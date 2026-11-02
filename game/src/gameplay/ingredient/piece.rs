use mplk_grid::bit_grid::{BitGrid, BitGridMask};

pub use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, sandbox_setup);
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

#[derive(Debug, Clone)]
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
            let floaty_tile = t.as_vec2();
            let x = pivot.x + pivot.y - floaty_tile.y;
            let y = pivot.y - pivot.x + floaty_tile.x;
            *t = TileCoords::new(x.round() as _, y.round() as _);
        }
    }

    pub fn rotate_ccw(&mut self) {
        todo!()
    }
}

fn sandbox_setup(mut cmd: Commands) {
    let piece = IngredientGridPiece {
        tiles: todo!(),
        pivot: todo!(),
    };
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
