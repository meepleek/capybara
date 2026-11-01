use bevy::prelude::*;
use rand::prelude::*;

pub trait RandExt {
    fn rotation(&mut self) -> Rot2;
    fn rotation_range_degrees(&mut self, degrees: f32) -> Rot2;
    fn direction(&mut self) -> Dir2;
}

impl RandExt for rand::prelude::ThreadRng {
    fn rotation(&mut self) -> Rot2 {
        self.rotation_range_degrees(360.0)
    }

    fn rotation_range_degrees(&mut self, degrees: f32) -> Rot2 {
        Rot2::degrees(self.random_range(-degrees..degrees))
    }

    fn direction(&mut self) -> Dir2 {
        Dir2::new(self.rotation() * Vec2::X).expect("Non-zero direction")
    }
}
