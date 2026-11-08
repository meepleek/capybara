use bevy::prelude::*;
use mplk_ext::quat::QuatExt;

#[derive(Default)]
pub struct AccumulateRotationPlugin;
impl Plugin for AccumulateRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, accumulate_angle);
    }
}

#[derive(Component, Debug, Default)]
pub struct AccumulatedRotation {
    prev: Option<Rot2>,
    pub rotation: f32,
}

fn accumulate_angle(mut acc_q: Query<(&mut AccumulatedRotation, &Transform), Changed<Transform>>) {
    for (mut acc, t) in &mut acc_q {
        let rot = t.rotation.to_rot2();
        if let Some(prev) = acc.prev {
            acc.rotation += prev.angle_to(rot);
        }
        acc.prev = Some(rot);
    }
}
