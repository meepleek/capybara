use mplk_ecs::observer::ensure_single_at_most;

use crate::gameplay::ingredient::INGREDIENT_TILE_SIZE_F32;
pub use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), sandbox_setup);
    app.add_observer(ensure_single_at_most::<PieceFocus>);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceFocus {
    Hover,
    Active,
}

#[derive(Debug, Clone, Copy)]
pub enum PiecePivot {
    Center(TileCoords),
    BtmRightCorner(TileCoords),
}
impl PiecePivot {
    pub fn center(center: impl Into<TileCoords>) -> Self {
        Self::Center(center.into())
    }

    pub fn btm_right_corner(corner: impl Into<TileCoords>) -> Self {
        Self::BtmRightCorner(corner.into())
    }

    pub fn as_vec2(self) -> Vec2 {
        match self {
            PiecePivot::Center(center) => center.as_vec2(),
            PiecePivot::BtmRightCorner(corner) => corner.as_vec2() + Vec2::splat(0.5),
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct IngredientGridPiece {
    pub tiles: Vec<TileCoords>,
    pub pivot: PiecePivot,
}
impl IngredientGridPiece {
    pub fn new<T: Into<TileCoords>>(pivot: PiecePivot, tiles: impl IntoIterator<Item = T>) -> Self {
        Self {
            tiles: tiles.into_iter().map(Into::into).collect(),
            pivot,
        }
    }

    pub fn rotate_cw(&mut self) {
        let pivot = self.pivot.as_vec2();
        for t in &mut self.tiles {
            let x = pivot.x + pivot.y - t.y as f32;
            let y = pivot.y - pivot.x + t.x as f32;
            *t = TileCoords::new(x.round() as _, y.round() as _);
        }
    }

    pub fn rotate_ccw(&mut self) {
        let pivot = self.pivot.as_vec2();
        for t in &mut self.tiles {
            let x = pivot.x - pivot.y + t.y as f32;
            let y = pivot.y + pivot.x - t.x as f32;
            *t = TileCoords::new(x.round() as _, y.round() as _);
        }
    }
}

fn sandbox_setup(mut cmd: Commands) {
    tracing::warn!("piece setup");
    let p = IngredientGridPiece::new(PiecePivot::center((1, 0)), [(0, 0), (1, 0), (2, 0), (1, 1)]);
    cmd.spawn_scene(piece(p, (-300., 250.)));
}

pub fn piece(piece: IngredientGridPiece, position: impl Into<Vec2>) -> impl Scene {
    let mut tiles: Vec<Box<dyn Scene>> = Vec::new();
    for t in &piece.tiles {
        let translation = (t.as_vec2() * INGREDIENT_TILE_SIZE_F32).extend(0.);
        let tile = bsn! {
            #piece_tile
            Transform { translation }
            Visibility::default()
            Pickable::default()
            Children [
                #piece_border
                Transform { translation: { Vec3::Z * 0.1 } }
                from_collection(|sprites: &SpriteAssets| Sprite {
                    image: sprites.sheet.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: sprites.layout.clone(),
                        index: 0
                    }),
                    image_mode: SpriteImageMode::Sliced(TextureSlicer {
                        border: BorderRect::all(5.),
                        center_scale_mode: SliceScaleMode::Tile {
                            stretch_value: 1. },
                        ..default()
                    }),
                    custom_size: Some(Vec2::splat(INGREDIENT_TILE_SIZE_F32)),
                    ..default()
                })
                --
                #piece_content
                Transform { translation: { Vec2::new(11., -11.).extend(0.) } }
                from_collection(|sprites: &SpriteAssets| Sprite {
                    image: sprites.sheet.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: sprites.layout.clone(),
                        index: 1
                    }),
                    custom_size: Some(Vec2::splat(89.)),
                    ..default()
                })
                --
                #piece_picking_overlay
                Mesh2d::from_collection(SpriteAssets::piece_picking_mesh)
                Pickable::default()
            ]
        };
        tiles.push(Box::new(tile));
    }

    let translation = position.into().extend(0.);
    bsn! {
        #piece
        Transform { translation }
        Visibility::default()
        piece
        DespawnOnExit::<Screen>(Screen::Gameplay)
        Children[{tiles}]
        Pickable::default()
        on(|ev: On<PointerEnter>, mut cmd: Commands| {
            if ev.original_event_target() == ev.entity {
                cmd.try_insert_to(ev.entity, PieceFocus::Hover);
            }
        })
        on(|ev: On<PointerLeave>, mut cmd: Commands| {
            cmd.try_remove_from::<PieceFocus>(ev.entity);
        })
        on(|ev: On<PointerPress>, mut cmd: Commands| {
            cmd.try_insert_to(ev.entity, PieceFocus::Active);
        })
        on(|ev: On<PointerRelease>, mut q: Query<&mut PieceFocus>| {
            let mut focus = or_return!(q.get_mut(ev.entity));
            if *focus == PieceFocus::Active {
                *focus = PieceFocus::Hover;
            }
        })
        on(|ev: On<PointerDragEnd>, mut q: Query<&mut PieceFocus>| {
            let mut focus = or_return!(q.get_mut(ev.entity));
            if *focus == PieceFocus::Active {
                *focus = PieceFocus::Hover;
            }
        })
        on(|ev: On<PointerDrag>, mut q: Query<&mut Transform>| {
            let mut t = or_return!(q.get_mut(ev.entity));
            let d = ev.delta;
            t.translation += Vec3::new(d.x, -d.y, 0.);
        })
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(test_piece_t(), 1 => vec![(1, -1), (1, 0), (1, 1), (0, 0)])]
    #[test_case(test_piece_t(), 2 => vec![(2, 0), (1, 0), (0, 0), (1, -1)])]
    #[test_case(test_piece_t(), 3 => vec![(1, 1), (1, 0), (1, -1), (2, 0)])]
    #[test_case(test_piece_t(), 4 => vec![(0, 0), (1, 0), (2, 0), (1, 1)])]
    #[test_case(test_piece_line_top_pivot(), 1 => vec![(0, 0), (0, 1), (0, 2)])]
    #[test_case(test_piece_line_top_pivot(), 2 => vec![(0, 0), (-1, 0), (-2, 0)])]
    #[test_case(test_piece_line_top_pivot(), 3 => vec![(0, 0), (0, -1), (0, -2)])]
    #[test_case(test_piece_line_top_pivot(), 4 => vec![(0, 0), (1, 0), (2, 0)])]
    #[test_case(test_piece_square(), 1 => vec![(1, 0), (1, 1), (0, 1), (0, 0)])]
    #[test_case(test_piece_square(), 2 => vec![(1, 1), (0, 1), (0, 0), (1, 0)])]
    #[test_case(test_piece_square(), 3 => vec![(0, 1), (0, 0), (1, 0), (1, 1)])]
    #[test_case(test_piece_square(), 4 => vec![(0, 0), (1, 0), (1, 1), (0, 1)])]
    #[test_case(test_piece_line(), 1 => vec![(1, 2), (0, 2), (-1, 2), (-2, 2)])]
    #[test_case(test_piece_line(), 2 => vec![(-1, 3), (-1, 2), (-1, 1), (-1, 0)])]
    #[test_case(test_piece_line(), 3 => vec![(-2, 1), (-1, 1), (0, 1), (1, 1)])]
    #[test_case(test_piece_line(), 4 => vec![(0, 0), (0, 1), (0, 2), (0, 3)])]
    fn rotate_cw(mut piece: IngredientGridPiece, rotation_count: usize) -> Vec<(i16, i16)> {
        for _ in 0..rotation_count {
            piece.rotate_cw();
        }

        piece.tiles.into_iter().map(|t| (t.x, t.y)).collect()
    }

    #[test_case(test_piece_t(), 1 => vec![(1, 1), (1, 0), (1, -1), (2, 0)])]
    #[test_case(test_piece_t(), 2 => vec![(2, 0), (1, 0), (0, 0), (1, -1)])]
    #[test_case(test_piece_t(), 3 => vec![(1, -1), (1, 0), (1, 1), (0, 0)])]
    #[test_case(test_piece_t(), 4 => vec![(0, 0), (1, 0), (2, 0), (1, 1)])]
    #[test_case(test_piece_line_top_pivot(), 1 => vec![(0, 0), (0, -1), (0, -2)])]
    #[test_case(test_piece_line_top_pivot(), 2 => vec![(0, 0), (-1, 0), (-2, 0)])]
    #[test_case(test_piece_line_top_pivot(), 3 => vec![(0, 0), (0, 1), (0, 2)])]
    #[test_case(test_piece_line_top_pivot(), 4 => vec![(0, 0), (1, 0), (2, 0)])]
    #[test_case(test_piece_square(), 1 => vec![(0, 1), (0, 0), (1, 0), (1, 1)])]
    #[test_case(test_piece_square(), 2 => vec![(1, 1), (0, 1), (0, 0), (1, 0)])]
    #[test_case(test_piece_square(), 3 => vec![(1, 0), (1, 1), (0, 1), (0, 0)])]
    #[test_case(test_piece_square(), 4 => vec![(0, 0), (1, 0), (1, 1), (0, 1)])]
    #[test_case(test_piece_line(), 1 => vec![(-2, 1), (-1, 1), (0, 1), (1, 1)])]
    #[test_case(test_piece_line(), 2 => vec![(-1, 3), (-1, 2), (-1, 1), (-1, 0)])]
    #[test_case(test_piece_line(), 3 => vec![(1, 2), (0, 2), (-1, 2), (-2, 2)])]
    #[test_case(test_piece_line(), 4 => vec![(0, 0), (0, 1), (0, 2), (0, 3)])]
    fn rotate_ccw(mut piece: IngredientGridPiece, rotation_count: usize) -> Vec<(i16, i16)> {
        for _ in 0..rotation_count {
            piece.rotate_ccw();
        }

        piece.tiles.into_iter().map(|t| (t.x, t.y)).collect()
    }

    fn test_piece_t() -> IngredientGridPiece {
        IngredientGridPiece::new(PiecePivot::center((1, 0)), [(0, 0), (1, 0), (2, 0), (1, 1)])
    }

    fn test_piece_line_top_pivot() -> IngredientGridPiece {
        IngredientGridPiece::new(PiecePivot::center((0, 0)), [(0, 0), (1, 0), (2, 0)])
    }

    fn test_piece_square() -> IngredientGridPiece {
        IngredientGridPiece::new(
            PiecePivot::btm_right_corner((0, 0)),
            [(0, 0), (1, 0), (1, 1), (0, 1)],
        )
    }

    fn test_piece_line() -> IngredientGridPiece {
        IngredientGridPiece::new(
            PiecePivot::btm_right_corner((-1, 1)),
            [(0, 0), (0, 1), (0, 2), (0, 3)],
        )
    }
}
