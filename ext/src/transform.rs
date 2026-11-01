use bevy::prelude::*;

pub trait TransformExt {
    fn zero_scale_2d() -> Transform;
}

impl TransformExt for Transform {
    fn zero_scale_2d() -> Transform {
        Transform::from_scale(Vec2::ZERO.extend(1.))
    }
}
