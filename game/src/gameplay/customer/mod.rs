use crate::{
    gameplay::{
        inventory::InventoryIndex,
        layout::{GameLayout, InventoryPosition},
    },
    prelude::*,
};

pub mod order;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((order::plugin, spawn::plugin));
}

#[derive(Component, FromTemplate)]
pub struct Customer;
impl InventoryPosition for Customer {
    fn inventory_position(layout: &GameLayout, idx: &InventoryIndex) -> Vec2 {
        layout.customer_position(idx.0)
    }
}

pub enum CustomerKind {
    Regular,
    Fast,
    Snobby,
    Critic,
    Tutorial,
}
