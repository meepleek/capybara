use mplk_ecs::observer::ensure_single_at_most;

use crate::{gameplay::piece::template::PieceTemplate, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(ensure_single_at_most::<PieceBag>);
}

#[derive(Debug, Clone)]
pub enum BagPieceTemplate {
    Template(PieceTemplate),
    Random,
}

#[derive(Component, Debug, Clone, Default)]
pub struct PieceBag {
    pieces: Vec<PieceTemplate>,
    random_count: u8,
    bag: Vec<BagPieceTemplate>,
}
impl PieceBag {
    pub fn new(pieces: impl Into<Vec<PieceTemplate>>, random_count: u8) -> Self {
        let mut bag = Self {
            bag: Vec::new(),
            pieces: pieces.into(),
            random_count,
        };
        bag.fill_bag();
        bag
    }

    pub fn choose(&mut self, rng: &mut impl Rng) -> PieceTemplate {
        if self.bag.is_empty() {
            self.fill_bag();
        }

        let idx = (0..self.bag.len())
            .choose(rng)
            .expect("bag should never be empty");
        match self.bag.swap_remove(idx) {
            BagPieceTemplate::Template(template) => template,
            BagPieceTemplate::Random => self.pieces.choose(rng).cloned().expect("invalid full bag"),
        }
    }

    fn fill_bag(&mut self) {
        let mut bag: Vec<_> = self
            .pieces
            .iter()
            .cloned()
            .map(BagPieceTemplate::Template)
            .collect();
        for _ in 0..self.random_count {
            bag.push(BagPieceTemplate::Random);
        }
        self.bag = bag;
    }
}
