//! Spawn the main level.

use crate::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((TraumaPlugin, MeshPickingPlugin));
    app.add_systems(Startup, spawn_camera);

    // todo: remove me
    // hacky workaround: https://github.com/bevyengine/bevy/issues/26362
    app.add_observer(force_sprite_mask_alpha);
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

// todo: remove me
// hacky workaround: https://github.com/bevyengine/bevy/issues/26362
fn force_sprite_mask_alpha(ev: On<Insert<Sprite>>, mut sprite_q: Query<&mut Sprite>) {
    let mut sprite = or_return!(sprite_q.get_mut(ev.entity));
    sprite.alpha_mode = bevy::sprite::SpriteAlphaMode::Mask(1.0);
}
