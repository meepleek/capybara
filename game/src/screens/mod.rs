//! The game's main screen states and transitions between them.

mod game_over;
mod gameplay;
mod splash;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((gameplay::plugin, splash::plugin, game_over::plugin));
}

/// The game's main screen states.
#[derive(clap::ValueEnum, States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    MainMenu,
    Tutorial,
    Gameplay,
    GameOver,
}

// pub fn is_in_game<TState: States>(state: TState) -> impl FnMut(Res<State<Screen>>) -> bool + Clone {
//     move |state| match state {
//         Some(fade_state) => match fade_state.get() {
//             FadeState::FadingIn(to_state) => *to_state == state,
//             _ => false,
//         },
//         None => false,
//     }
// }
