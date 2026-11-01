use bevy::prelude::*;

pub trait Dir2Ext {
    fn to_quat(self) -> Quat;
}

impl Dir2Ext for Dir2 {
    fn to_quat(self) -> Quat {
        Quat::from_rotation_z(self.to_angle())
    }
}
