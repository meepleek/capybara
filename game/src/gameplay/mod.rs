use bevy::prelude::*;

mod camera;
pub mod capy;
pub mod cauldron;
pub mod customer;
pub mod drag;
pub mod grid;
pub mod ingredient;
pub mod inventory;
pub mod layout;
pub mod level;
pub mod piece;
pub mod turn;
pub mod ui;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        camera::plugin,
        capy::plugin,
        ingredient::plugin,
        grid::plugin,
        piece::plugin,
        drag::plugin,
        cauldron::plugin,
        layout::plugin,
        customer::plugin,
        level::plugin,
        ui::plugin,
    ));
}
