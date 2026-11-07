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

#[derive(Component, Debug, Clone, Copy, Default)]
pub enum CustomerKind {
    #[default]
    Regular,
    Fast,
    Snobby,
    Critic,
}
impl CustomerKind {
    pub fn regulars() -> [CustomerKind; 3] {
        [CustomerKind::Regular; 3]
    }
}
