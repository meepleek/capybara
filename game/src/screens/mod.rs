use bevy::prelude::*;

mod game_over;
mod gameplay;
pub mod run_conditions;
mod splash;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>().add_sub_state::<GameplayLevel>();

    app.add_plugins((gameplay::plugin, splash::plugin, game_over::plugin));
}

#[derive(clap::ValueEnum, States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    MainMenu,
    Tutorial,
    Gameplay,
    GameOver,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(Screen = Screen::Gameplay)]
pub enum GameplayLevel {
    #[default]
    Regular,
    Tutorial,
}
