//! Spawn the main level.

use crate::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TraumaPlugin);
    app.add_systems(Startup, spawn_camera);
}

#[derive(Component)]
pub struct PrimaryCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        PrimaryCamera,
        Shake::default(),
    ));
}
