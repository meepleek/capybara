use bevy::prelude::*;

pub trait Rot2Ext {
    fn to_quat(self) -> Quat;
}

impl Rot2Ext for Rot2 {
    fn to_quat(self) -> Quat {
        Quat::from_rotation_z(self.as_radians())
    }
}

impl Rot2Ext for Dir2 {
    fn to_quat(self) -> Quat {
        Quat::from_rotation_z(self.to_angle())
    }
}
