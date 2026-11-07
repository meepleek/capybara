use mplk_ecs::observer::ensure_single_at_most;

use crate::{
    gameplay::{
        customer::{
            CustomerKind,
            order::{OrderFinished, OrderMissed},
        },
        piece::{AddPiece, INVENTORY_MAX, Piece, bag::PieceBag, template::PieceTemplate},
    },
    prelude::*,
    screens::run_conditions::in_game,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(processed_missed_order)
        .add_observer(processed_finished_order)
        .add_observer(ensure_single_at_most::<LevelData>);
    app.add_systems(Update, spawn_pieces.run_if(in_game));
}

#[derive(Component, Debug, Clone)]
pub struct LevelData {
    pub piece_bag: Vec<PieceTemplate>,
    pub piece_bag_random_count: u8,
    pub concurrent_order_max: u8,
    pub order_count: u8,
    pub customers: [CustomerKind; 3],
}
impl LevelData {
    pub fn basics() -> Self {
        LevelData {
            piece_bag: PieceTemplate::classic_bag(),
            piece_bag_random_count: 1,
            concurrent_order_max: 1,
            order_count: 5,
            customers: CustomerKind::regulars(),
        }
    }
}

fn processed_finished_order(_ev: On<OrderFinished>) {
    // todo
}

fn processed_missed_order(_ev: On<OrderMissed>, mut next: ResMut<NextState<Screen>>) {
    next.set(Screen::GameOver);
}

fn spawn_pieces(piece_q: Query<&Piece>, mut cmd: Commands, mut piece_bag: Single<&mut PieceBag>) {
    let mut rng = rng();
    if piece_q.is_empty() {
        for i in 0..INVENTORY_MAX {
            let piece = piece_bag.choose(&mut rng);
            let cw_rotations = rng.random_range(0..=3);
            cmd.trigger(AddPiece {
                piece,
                cw_rotations,
                idx: Some(i),
            });
        }
    }
}
