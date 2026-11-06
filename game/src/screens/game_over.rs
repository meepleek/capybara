use crate::{prelude::*, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::GameOver),
        spawn_ui.run_if(in_state(Screen::GameOver)),
    );
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("game over"),
        DespawnOnExit(Screen::GameOver),
        children![
            widget::header("game over ):"),
            widget::button("Try again", play),
        ],
    ));
}

fn play(_: On<PointerClick>, mut next_menu: ResMut<NextState<Screen>>) {
    next_menu.set(Screen::Gameplay);
}
