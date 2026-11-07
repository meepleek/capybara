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
