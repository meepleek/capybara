use bevy::prelude::*;

mod camera;
pub mod ingredient;
pub mod turn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, ingredient::plugin));
}
