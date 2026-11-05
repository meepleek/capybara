use bevy::prelude::*;

mod camera;
pub mod drag;
pub mod grid;
pub mod ingredient;
pub mod inventory;
pub mod layout;
pub mod piece;
pub mod turn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        camera::plugin,
        ingredient::plugin,
        grid::plugin,
        piece::plugin,
        drag::plugin,
    ));
}
