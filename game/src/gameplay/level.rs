use crate::{gameplay::customer::order::OrderMissed, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(processed_missed_order);
}

fn processed_missed_order(_ev: On<OrderMissed>, mut next: ResMut<NextState<Screen>>) {
    next.set(Screen::GameOver);
}
