use crate::{gameplay::cauldron::OrderlessDrink, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(process_orderless_drinks);
}

fn process_orderless_drinks(ev: On<OrderlessDrink>) {
    // todo: capy gives out trivia
    tracing::warn!(ev=?ev.event(), "orderless drink");
}
