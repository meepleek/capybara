use mplk_ecs::{hiearchy::RotationOf, observer::ensure_single_at_most};
use mplk_utils::math::asymptotic_smoothing_with_delta_time;

use crate::gameplay::{
    ingredient::{
        INGREDIENT_TILE_SIZE_F32,
        grid::{GridSection, IngredientGrid},
        piece_template::PieceTemplate,
    },
    layout::GameLayout,
};
pub use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_pieces.run_if(in_state(Screen::Gameplay)));
    app.add_systems(
        Update,
        snap_pieces_to_grid.run_if(in_state(Screen::Gameplay)),
    );
    app.add_observer(ensure_single_at_most::<PieceFocus>);
}

const PIECE_COUNT: u8 = 3;

#[derive(Component, Debug, Clone, PartialEq, Eq, FromTemplate)]
pub struct PieceInventoryIndex(u8);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceFocus {
    Hover,
    Active,
}

#[derive(Component, Debug)]
pub struct IgnoreDrag;

#[derive(Debug, Component, Clone, Default)]
#[require(Transform)]
struct PieceDragPosition {
    drag_position: Vec2,
    snapped_position: Option<Vec2>,
}
impl PieceDragPosition {
    pub fn new(drag_position: Vec2) -> Self {
        Self {
            drag_position,
            snapped_position: None,
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct Piece {
    pub tiles: Vec<TileCoords>,
}
impl Piece {
    pub fn new<T: Into<TileCoords>>(tiles: impl IntoIterator<Item = T>) -> Self {
        Self {
            tiles: tiles.into_iter().map(Into::into).collect(),
        }
    }

    pub fn rotate_cw(&mut self) {
        for t in &mut self.tiles {
            *t = TileCoords::new(-t.y, t.x);
        }
        self.normalize_tiles();
    }

    pub fn rotate_ccw(&mut self) {
        for t in &mut self.tiles {
            *t = TileCoords::new(t.y, -t.x);
        }
        self.normalize_tiles();
    }

    fn normalize_tiles(&mut self) {
        if self.tiles.is_empty() {
            return;
        }

        let min_x = self
            .tiles
            .iter()
            .min_by_key(|t| t.x)
            .expect("failed to find min x")
            .x;
        let min_y = self
            .tiles
            .iter()
            .min_by_key(|t| t.y)
            .expect("failed to find min y")
            .y;
        let offset = TileCoords::new(-min_x, -min_y);
        for t in &mut self.tiles {
            *t += offset;
        }
        // todo: pivot as well?
        // todo!()
    }
}

fn spawn_pieces(piece_q: Query<&Piece>, mut cmd: Commands, layout: GameLayout) {
    let mut rng = rng();
    if piece_q.is_empty() {
        let lvl_pieces = [
            PieceTemplate::Line2,
            PieceTemplate::Line3,
            PieceTemplate::Line4,
            PieceTemplate::T,
            PieceTemplate::Square,
            PieceTemplate::Corner,
            PieceTemplate::L,
            PieceTemplate::LFlip,
            PieceTemplate::Z,
            PieceTemplate::ZFlip,
        ];
        for i in 0..PIECE_COUNT {
            let template = lvl_pieces
                .choose(&mut rng)
                .expect("No valid piece template");
            let mut p = template.piece();
            for _ in 0..(rng.random_range(0..=3)) {
                p.rotate_cw();
            }
            let position = layout.piece_position(i);
            cmd.spawn_scene(piece_scene(p, position, i));
        }
    }
}

pub fn piece_scene(piece: Piece, position: impl Into<Vec2>, inventory_idx: u8) -> impl Scene {
    let mut tiles: Vec<Box<dyn Scene>> = Vec::new();
    for t in &piece.tiles {
        let translation = (t.as_world_vec2() * INGREDIENT_TILE_SIZE_F32).extend(0.);
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

    let position = position.into();
    let translation = position.extend(0.);
    bsn! {
        #piece_root
        piece
        Transform { translation }
        PieceInventoryIndex(inventory_idx)
        PieceDragPosition::new(position)
        Visibility
        DespawnOnExit::<Screen>(Screen::Gameplay)
        Pickable
        on(|ev: On<PointerEnter>, focus_q: Query<(), With<PieceFocus>>, mut cmd: Commands| {
            if ev.original_event_target() == ev.entity && !focus_q.contains(ev.entity)
             {
                cmd.try_insert_to(ev.entity, PieceFocus::Hover);
            }
        })
        on(|ev: On<PointerLeave>, focus_q: Query<&PieceFocus, Without<IgnoreDrag>>, mut cmd: Commands| {
            let focus = or_return!(focus_q.get(ev.entity));
            if *focus == PieceFocus::Hover {
                cmd.try_remove_from::<PieceFocus>(ev.entity);
            }
        })
        on(|ev: On<PointerPress>, mut cmd: Commands, ignore_q: Query<(), With<IgnoreDrag>>| {
            if ignore_q.contains(ev.entity) {
                return;
            }
            cmd.try_insert_to(ev.entity, PieceFocus::Active);
        })
        on(on_drag_end)
        on(|ev: On<PointerDrag>, mut q: Query<&mut PieceDragPosition>, focus_q: Query<(&PieceFocus, Has<IgnoreDrag>)>| {
            let (focus, ignore_drag) = or_return!(focus_q.get(ev.entity));
            if *focus != PieceFocus::Active || ignore_drag {
                return;
            }
            let mut drag = or_return!(q.get_mut(ev.entity));
            let d = ev.delta;
            drag.drag_position += Vec2::new(d.x, -d.y);
        })
        Children[
            #piece_sprites
            Visibility
            Transform
            RotationOf(#piece)
            Children [{tiles}]
        ]
    }
}

fn on_drag_end(
    ev: On<PointerDragEnd>,
    mut piece_q: Query<
        (
            &mut Transform,
            &Piece,
            &mut PieceDragPosition,
            &PieceInventoryIndex,
        ),
        Without<IgnoreDrag>,
    >,
    mut grid: Single<&mut IngredientGrid>,
    mut cmd: Commands,
    layout: GameLayout,
) {
    let (mut t, piece, mut drag, piece_idx) = or_return!(piece_q.get_mut(ev.entity));
    if let Some(snapped_pos) = drag.snapped_position
        && let Some(tile) = grid.world_to_tile(snapped_pos)
    {
        let completed_sections = or_return!(grid.place_piece(piece, tile));
        tracing::warn!(?completed_sections);
        for cs in completed_sections {
            match cs {
                GridSection::Column(col) => or_continue!(grid.clear_col(col)),
                GridSection::Row(row) => or_continue!(grid.clear_row(row)),
                GridSection::Area(idx) => or_continue!(grid.clear_area(idx)),
            };
        }
        // set position in case it's still easing
        t.translation = snapped_pos.extend(t.translation.z);
        // todo: tween out
        cmd.try_remove_from::<Piece>(ev.entity);
        cmd.try_insert_to(
            ev.entity,
            TransformScaleLensSrc::new(Vec2::ZERO)
                .duration(ms(160))
                .despawn_target_on_completion(),
        );
    } else {
        cmd.try_remove_from::<PieceFocus>(ev.entity);
        let start_pos = layout.piece_position(piece_idx.0);
        drag.drag_position = start_pos;
        let tween_duration = ms(330);
        cmd.try_insert_to(
            ev.entity,
            (
                TransformPositionLensSrc::new(start_pos)
                    .duration(tween_duration)
                    .easing(EaseFunction::BackOut),
                IgnoreDrag,
            ),
        );
        cmd.delayed()
            .duration(tween_duration)
            .entity(ev.entity)
            .try_remove::<IgnoreDrag>();
    }
}

fn snap_pieces_to_grid(
    mut piece_q: Query<(&mut Transform, &Piece, &mut PieceDragPosition), With<PieceFocus>>,
    grid: Single<&IngredientGrid>,
    time: Res<Time>,
) {
    for (mut t, piece, mut drag) in &mut piece_q {
        if let Some(tile) = grid.world_to_tile(drag.drag_position)
            && let Ok(_completed_sections) = grid.preview_piece_placement(piece, tile)
        {
            let tile_pos = or_continue!(grid.tile_to_world(tile));
            drag.snapped_position = Some(tile_pos);
        } else {
            drag.snapped_position = None;
        }

        let target = drag.snapped_position.unwrap_or(drag.drag_position);
        t.translation = asymptotic_smoothing_with_delta_time(
            t.translation,
            target.extend(t.translation.z),
            0.4,
            time.delta_secs(),
        );
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(test_piece_t(), 1 => vec![(1, 0), (1, 1), (1, 2), (0, 1)])]
    #[test_case(test_piece_t(), 2 => vec![(2, 1), (1, 1), (0, 1), (1, 0)])]
    #[test_case(test_piece_t(), 3 => vec![(0, 2), (0, 1), (0, 0), (1, 1)])]
    #[test_case(test_piece_t(), 4 => vec![(0, 0), (1, 0), (2, 0), (1, 1)])]
    #[test_case(test_piece_line_top_pivot(), 1 => vec![(0, 0), (0, 1), (0, 2)])]
    #[test_case(test_piece_line_top_pivot(), 2 => vec![(2, 0), (1, 0), (0, 0)])]
    #[test_case(test_piece_line_top_pivot(), 3 => vec![(0, 2), (0, 1), (0, 0)])]
    #[test_case(test_piece_line_top_pivot(), 4 => vec![(0, 0), (1, 0), (2, 0)])]
    #[test_case(test_piece_square(), 1 => vec![(1, 0), (1, 1), (0, 1), (0, 0)])]
    #[test_case(test_piece_square(), 2 => vec![(1, 1), (0, 1), (0, 0), (1, 0)])]
    #[test_case(test_piece_square(), 3 => vec![(0, 1), (0, 0), (1, 0), (1, 1)])]
    #[test_case(test_piece_square(), 4 => vec![(0, 0), (1, 0), (1, 1), (0, 1)])]
    #[test_case(test_piece_line(), 1 => vec![(3, 0), (2, 0), (1, 0), (0, 0)])]
    #[test_case(test_piece_line(), 2 => vec![(0, 3), (0, 2), (0, 1), (0, 0)])]
    #[test_case(test_piece_line(), 3 => vec![(0, 0), (1, 0), (2, 0), (3, 0)])]
    #[test_case(test_piece_line(), 4 => vec![(0, 0), (0, 1), (0, 2), (0, 3)])]
    fn rotate_cw(mut piece: Piece, rotation_count: usize) -> Vec<(i16, i16)> {
        for _ in 0..rotation_count {
            piece.rotate_cw();
        }

        piece.tiles.into_iter().map(|t| (t.x, t.y)).collect()
    }

    #[test_case(test_piece_t(), 1 => vec![(0, 2), (0, 1), (0, 0), (1, 1)])]
    #[test_case(test_piece_t(), 2 => vec![(2, 1), (1, 1), (0, 1), (1, 0)])]
    #[test_case(test_piece_t(), 3 => vec![(1, 0), (1, 1), (1, 2), (0, 1)])]
    #[test_case(test_piece_t(), 4 => vec![(0, 0), (1, 0), (2, 0), (1, 1)])]
    #[test_case(test_piece_line_top_pivot(), 1 => vec![(0, 2), (0, 1), (0, 0)])]
    #[test_case(test_piece_line_top_pivot(), 2 => vec![(2, 0), (1, 0), (0, 0)])]
    #[test_case(test_piece_line_top_pivot(), 3 => vec![(0, 0), (0, 1), (0, 2)])]
    #[test_case(test_piece_line_top_pivot(), 4 => vec![(0, 0), (1, 0), (2, 0)])]
    #[test_case(test_piece_square(), 1 => vec![(0, 1), (0, 0), (1, 0), (1, 1)])]
    #[test_case(test_piece_square(), 2 => vec![(1, 1), (0, 1), (0, 0), (1, 0)])]
    #[test_case(test_piece_square(), 3 => vec![(1, 0), (1, 1), (0, 1), (0, 0)])]
    #[test_case(test_piece_square(), 4 => vec![(0, 0), (1, 0), (1, 1), (0, 1)])]
    #[test_case(test_piece_line(), 1 => vec![(0, 0), (1, 0), (2, 0), (3, 0)])]
    #[test_case(test_piece_line(), 2 => vec![(0, 3), (0, 2), (0, 1), (0, 0)])]
    #[test_case(test_piece_line(), 3 => vec![(3, 0), (2, 0), (1, 0), (0, 0)])]
    #[test_case(test_piece_line(), 4 => vec![(0, 0), (0, 1), (0, 2), (0, 3)])]
    fn rotate_ccw(mut piece: Piece, rotation_count: usize) -> Vec<(i16, i16)> {
        for _ in 0..rotation_count {
            piece.rotate_ccw();
        }

        piece.tiles.into_iter().map(|t| (t.x, t.y)).collect()
    }

    fn test_piece_t() -> Piece {
        PieceTemplate::T.piece()
    }

    fn test_piece_square() -> Piece {
        PieceTemplate::Square.piece()
    }

    fn test_piece_line() -> Piece {
        PieceTemplate::Line4.piece()
    }

    fn test_piece_line_top_pivot() -> Piece {
        Piece::new([(0, 0), (1, 0), (2, 0)])
    }
}
