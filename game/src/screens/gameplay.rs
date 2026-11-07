use crate::prelude::*;

pub(super) fn plugin(_app: &mut App) {
    // app.add_systems(
    //     Update,
    //     (
    //         (pause, spawn_pause_overlay, open_pause_menu).run_if(
    //             in_game)
    //                 .and_then(in_state(Menu::None))
    //                 .and_then(input_just_pressed(KeyCode::Escape)),
    //         ),
    //         close_menu.run_if(
    //             in_game)
    //                 .and_then(not(in_state(Menu::None)))
    //                 .and_then(input_just_pressed(KeyCode::Escape)),
    //         ),
    //     ),
    // );
    // app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause));
    // app.add_systems(OnEnter(Menu::None), unpause.run_if(in_game));
}

// fn unpause(mut next_pause: ResMut<NextState<Pause>>) {
//     next_pause.set(Pause(false));
// }

// fn pause(mut next_pause: ResMut<NextState<Pause>>) {
//     next_pause.set(Pause(true));
// }

// fn spawn_pause_overlay(mut commands: Commands) {
//     commands.spawn((
//         Name::new("Pause Overlay"),
//         Node {
//             width: percent(100),
//             height: percent(100),
//             ..default()
//         },
//         GlobalZIndex(1),
//         BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
//         DespawnOnExit(Pause(true)),
//     ));
// }

// fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
//     next_menu.set(Menu::Pause);
// }

// fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
//     next_menu.set(Menu::None);
// }
