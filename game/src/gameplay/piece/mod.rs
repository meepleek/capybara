mod drag;
pub(crate) mod spawn;
pub(crate) mod template;

use crate::{
    gameplay::{
        inventory::InventoryIndex,
        layout::{GameLayout, InventoryPosition},
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((spawn::plugin, drag::plugin));
}

pub(crate) const INVENTORY_MAX: u8 = 3;

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
impl InventoryPosition for Piece {
    fn inventory_position(layout: &GameLayout, idx: &InventoryIndex) -> Vec2 {
        layout.piece_position(idx.0)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::gameplay::piece::template::PieceTemplate;

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
