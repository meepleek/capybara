use crate::{
    gameplay::{
        customer::{
            CustomerKind,
            order::{OrderFinished, OrderMissed},
        },
        piece::template::PieceTemplate,
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(processed_missed_order)
        .add_observer(processed_finished_order);
    // app.add_systems(Update, respawn_pieces.run_if(in_game));
}

#[derive(Debug, Clone)]
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

// fn respawn_pieces(piece_q: Query<&Piece>, mut cmd: Commands, layout: GameLayout) {
//     use PieceTemplate::*;

//     let mut rng = rng();
//     if piece_q.is_empty() {
//         let lvl_pieces = [Line2, Line3, Line4, T, Square, Corner, L, LFlip, Z, ZFlip];
//         for i in 0..INVENTORY_MAX {
//             let template = lvl_pieces
//                 .choose(&mut rng)
//                 .expect("No valid piece template");
//             let mut p = template.piece();
//             for _ in 0..(rng.random_range(0..=3)) {
//                 p.rotate_cw();
//             }
//             // todo: use AddPiece event
//         }
//     }
// }
