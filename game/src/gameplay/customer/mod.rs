use std::ops::RangeInclusive;

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
    // Critic,
}
impl CustomerKind {
    pub fn order_duration(self) -> Duration {
        sec(match self {
            CustomerKind::Regular => 90.,
            CustomerKind::Fast => 50.,
            CustomerKind::Snobby => 150.,
        })
    }

    pub fn ingredient_rank(self) -> RangeInclusive<u8> {
        match self {
            CustomerKind::Regular => 1..=5,
            CustomerKind::Fast => 1..=2,
            CustomerKind::Snobby => 4..=7,
        }
    }

    pub fn order_fused_ingredients(self) -> bool {
        match self {
            CustomerKind::Regular | CustomerKind::Snobby => true,
            CustomerKind::Fast => false,
        }
    }

    pub fn regulars() -> [CustomerKind; 3] {
        [CustomerKind::Regular; 3]
    }
}
