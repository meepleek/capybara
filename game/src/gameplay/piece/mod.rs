use crate::{
    gameplay::{
        drag::ValidDragTarget,
        ingredient::{CommonIngredient, FusedIngredient, Ingredient},
        inventory::InventoryIndex,
        layout::{GameLayout, InventoryPosition},
        piece::template::PieceTemplate,
    },
    prelude::*,
};

pub(crate) mod ability;
pub mod bag;
mod drag;
pub(crate) mod spawn;
pub(crate) mod template;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((spawn::plugin, drag::plugin, bag::plugin, ability::plugin));
}

pub(crate) const INVENTORY_MAX: u8 = 3;

#[derive(Event)]
pub struct AddPiece {
    pub piece: PieceTemplate,
    pub cw_rotations: u8,
    pub idx: Option<u8>,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceRotation {
    CW(u8),
    CCW(u8),
}

#[derive(EntityEvent, Debug)]
pub struct RotatePiece {
    pub entity: Entity,
    pub rotation: PieceRotation,
}

#[derive(Debug, Clone, Component)]
pub struct Piece {
    pub tiles: Vec<TileCoords>,
    pub center: Vec2,
    rotations: i32,
}
impl Piece {
    pub fn new<T: Into<TileCoords>>(
        tiles: impl IntoIterator<Item = T>,
        center: impl Into<Vec2>,
    ) -> Self {
        let mut piece = Self {
            tiles: tiles.into_iter().map(Into::into).collect(),
            center: center.into(),
            rotations: 0,
        };
        piece.normalize_tiles();
        piece
    }

    pub fn rotate_cw(&mut self) {
        for t in &mut self.tiles {
            *t = TileCoords::new(-t.y, t.x);
        }
        self.normalize_tiles();
        self.rotations -= 1;
        self.flip_center();
    }

    pub fn rotate_ccw(&mut self) {
        for t in &mut self.tiles {
            *t = TileCoords::new(t.y, -t.x);
        }
        self.normalize_tiles();
        self.rotations += 1;
        self.flip_center();
    }

    pub fn rotation_degrees(&self) -> f32 {
        self.rotations as f32 * 90.
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
    }

    fn flip_center(&mut self) {
        self.center = -self.center.yx();
    }
}
impl InventoryPosition for Piece {
    fn inventory_position(layout: &GameLayout, idx: &InventoryIndex) -> Vec2 {
        layout.piece_position(idx.0)
    }
}
impl ValidDragTarget<Ingredient> for Piece {
    fn valid(ingredient: &Ingredient) -> bool {
        use CommonIngredient::*;
        use FusedIngredient::*;
        use Ingredient::*;

        match ingredient {
            Common(Apple) | Fused(Pineapple) | Common(Orange) | Fused(Lime) => true,
            _ => false,
        }
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
}
