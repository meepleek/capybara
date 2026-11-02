use crate::gameplay::ingredient::grid::IngredientGrid;
pub use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), sandbox_setup);
    app.add_systems(Update, draw_piece_gizmos);
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

    fn as_vec2(self) -> Vec2 {
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
    let piece =
        IngredientGridPiece::new(PiecePivot::center((1, 0)), [(0, 0), (1, 0), (2, 0), (1, 1)]);
    cmd.spawn((Transform::from_xyz(-100., -100., 0.), piece));
}

fn draw_piece_gizmos(
    grid: Single<&IngredientGrid>,
    piece_q: Query<(&GlobalTransform, &IngredientGridPiece)>,
    mut gizmos: Gizmos,
) {
    let floaty_tile_size = grid.tile_size() as f32;
    for (piece_t, piece) in &piece_q {
        for t in &piece.tiles {
            gizmos.rounded_rect_2d(
                Isometry2d::from_translation(
                    piece_t.translation().truncate() + t.as_vec2() * floaty_tile_size,
                ),
                Vec2::splat(floaty_tile_size),
                bevy::color::palettes::tailwind::AMBER_600,
            );
        }
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
