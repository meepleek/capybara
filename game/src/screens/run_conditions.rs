use bevy::prelude::*;

use crate::screens::{GameplayLevel, Screen};

pub fn in_game(state: Res<State<Screen>>) -> bool {
    *state == Screen::Gameplay
}

pub fn in_tutorial_level(state: Res<State<GameplayLevel>>) -> bool {
    *state == GameplayLevel::Tutorial
}

pub fn in_regular_level(state: Res<State<GameplayLevel>>) -> bool {
    *state == GameplayLevel::Regular
}

// pub fn in_game<TState: States>(state: TState) -> impl FnMut(Res<State<Screen>>) -> bool + Clone {
//     move |state| *state == Screen::Gameplay
// }
