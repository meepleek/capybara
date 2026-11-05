use crate::{gameplay::piece::Piece, prelude::*};

#[expect(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum PieceTemplate {
    Line2,
    Line3,
    Line4,
    Square,
    Corner,
    L,
    LFlip,
    Z,
    ZFlip,
    T,
    Cubby, // hugger
    Slant2,
    Slant3,
    Cross,
    X,
}
impl PieceTemplate {
    pub fn piece(self) -> Piece {
        match self {
            PieceTemplate::Line2 => Piece::new([(0, 0), (0, 1)]),
            PieceTemplate::Line3 => Piece::new([(0, 0), (0, 1), (0, 2)]),
            PieceTemplate::Line4 => Piece::new([(0, 0), (0, 1), (0, 2), (0, 3)]),
            PieceTemplate::Square => Piece::new([(0, 0), (1, 0), (1, 1), (0, 1)]),
            PieceTemplate::Corner => Piece::new([(0, 0), (1, 0), (0, 1)]),
            PieceTemplate::L => Piece::new([(0, 0), (1, 0), (0, 1), (0, 2)]),
            PieceTemplate::LFlip => Piece::new([(0, 0), (1, 0), (1, 1), (1, 2)]),
            PieceTemplate::Z => Piece::new([(0, 0), (1, 0), (1, 1), (2, 1)]),
            PieceTemplate::ZFlip => Piece::new([(2, 0), (1, 0), (1, 1), (0, 1)]),
            PieceTemplate::T => Piece::new([(0, 0), (1, 0), (2, 0), (1, 1)]),
            PieceTemplate::Cubby => Piece::new([(0, 1), (0, 0), (1, 0), (2, 0), (2, 1)]),
            PieceTemplate::Slant2 => Piece::new([(0, 0), (1, 1)]),
            PieceTemplate::Slant3 => Piece::new([(0, 0), (1, 1), (2, 2)]),
            PieceTemplate::Cross => Piece::new([(1, 0), (0, 1), (1, 1), (1, 2), (2, 1)]),
            PieceTemplate::X => Piece::new([(0, 0), (2, 0), (1, 1), (0, 2), (2, 2)]),
        }
    }
}
