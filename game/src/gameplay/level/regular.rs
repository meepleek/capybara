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

#[expect(dead_code)]
#[derive(Debug, Clone)]
pub struct LevelData {
    piece_bag: Vec<PieceTemplate>,
    piece_bag_random_count: u8,
    concurrent_order_max: u8,
    order_count: u8,
    customers: [CustomerKind; 3],
    cauldron_count: u8,
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
