//! The game's main screen states and transitions between them.

mod gameplay;
mod splash;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((gameplay::plugin, splash::plugin));
}

/// The game's main screen states.
#[derive(clap::ValueEnum, States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    MainMenu,
    Tutorial,
    Gameplay,
}
