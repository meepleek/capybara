use bevy::prelude::*;
use mplk_ext::quat::QuatExt;

#[derive(Default)]
pub struct AccumulateRotationPlugin;
impl Plugin for AccumulateRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, accumulate_angle);
    }
}

#[derive(Component, Debug, Clone, Default)]
pub struct AccumulatedRotation {
    prev: Option<Rot2>,
    pub rotation_rad: f32,
}
impl AccumulatedRotation {
    pub fn new(rotation_rad: f32) -> Self {
        Self {
            rotation_rad,
            prev: None,
        }
    }
}

fn accumulate_angle(mut acc_q: Query<(&mut AccumulatedRotation, &Transform), Changed<Transform>>) {
    for (mut acc, t) in &mut acc_q {
        let rot = t.rotation.to_rot2();
        if let Some(prev) = acc.prev {
            acc.rotation_rad += prev.angle_to(rot);
        }
        acc.prev = Some(rot);
    }
}
