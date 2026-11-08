use crate::{gameplay::piece::Piece, prelude::*};

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
    Diag2,
    Diag3,
    Cross,
    X,
    Donut,
}
impl PieceTemplate {
    pub fn piece(self) -> Piece {
        match self {
            PieceTemplate::Line2 => Piece::new([(0, 0), (0, 1)], (0., 0.5)),
            PieceTemplate::Line3 => Piece::new([(0, 0), (0, 1), (0, 2)], (0., 1.)),
            PieceTemplate::Line4 => Piece::new([(0, 0), (0, 1), (0, 2), (0, 3)], (0., 1.5)),
            PieceTemplate::Square => Piece::new([(0, 0), (1, 0), (1, 1), (0, 1)], (-0.5, 0.5)),
            PieceTemplate::Corner => Piece::new([(0, 0), (1, 0), (0, 1)], (-0.5, 0.5)),
            PieceTemplate::L => Piece::new([(0, 0), (1, 0), (0, 1), (0, 2)], (-0.5, 1.)),
            PieceTemplate::LFlip => Piece::new([(0, 0), (1, 0), (1, 1), (1, 2)], (-0.5, 1.)),
            PieceTemplate::Z => Piece::new([(0, 0), (1, 0), (1, 1), (2, 1)], (-1., 0.5)),
            PieceTemplate::ZFlip => Piece::new([(2, 0), (1, 0), (1, 1), (0, 1)], (-1., 0.5)),
            PieceTemplate::T => Piece::new([(0, 0), (1, 0), (2, 0), (1, 1)], (-1., 0.5)),
            PieceTemplate::Cubby => {
                Piece::new([(0, 1), (0, 0), (1, 0), (2, 0), (2, 1)], (-1., 0.5))
            }
            PieceTemplate::Diag2 => Piece::new([(0, 0), (1, 1)], (-0.5, 0.5)),
            PieceTemplate::Diag3 => Piece::new([(0, 0), (1, 1), (2, 2)], (-1., 1.)),
            PieceTemplate::Cross => Piece::new([(1, 0), (0, 1), (1, 1), (1, 2), (2, 1)], (-1., 1.)),
            PieceTemplate::X => Piece::new([(0, 0), (2, 0), (1, 1), (0, 2), (2, 2)], (-1., 1.)),
            PieceTemplate::Donut => Piece::new(
                [
                    (1, 0),
                    (2, 0),
                    (2, 1),
                    (2, 2),
                    (1, 2),
                    (0, 2),
                    (0, 1),
                    (0, 0),
                ],
                (-1., 1.),
            ),
        }
    }

    pub fn classic_bag() -> Vec<Self> {
        use PieceTemplate::*;

        vec![Line4, Square, T, L, LFlip, Z, ZFlip]
    }

    pub fn common_bag() -> Vec<Self> {
        use PieceTemplate::*;

        let mut bag = Self::classic_bag();
        bag.extend_from_slice(&[Line2, Line3, Corner]);
        bag
    }

    pub fn ortho_lines_bag() -> Vec<Self> {
        use PieceTemplate::*;

        vec![Line2, Line3, Line4, Cross]
    }

    pub fn diag_lines_bag() -> Vec<Self> {
        use PieceTemplate::*;

        vec![Diag2, Diag3, X]
    }

    pub fn weird_bag() -> Vec<Self> {
        use PieceTemplate::*;

        vec![Corner, Cubby, Cross, X, Donut]
    }
}
