pub(super) use crate::{GAME_NAME, prelude::*};
use bevy::{
    settings::{SaveSettingsDeferred, SaveSettingsSync, SettingsPlugin},
    window::WindowCloseRequested,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(SettingsPlugin::new(&format!(
        "com.meepleek.{}",
        GAME_NAME.to_lowercase()
    )));
    app.add_systems(Update, save_on_window_close);
}

pub fn save_settings<TSettingsGroup: Resource>(
    settings_group: Res<TSettingsGroup>,
    mut cmd: Commands,
) {
    if settings_group.is_changed() && !settings_group.is_added() {
        cmd.queue(SaveSettingsDeferred(ms(300)));
        tracing::warn!("save settings")
    }
}

fn save_on_window_close(
    mut win_close_mr: MessageReader<WindowCloseRequested>,
    mut commands: Commands,
) {
    if win_close_mr.clear_any() {
        commands.queue(SaveSettingsSync::IfChanged);
    }
}
