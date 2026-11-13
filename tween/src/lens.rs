use bevy::prelude::*;
use bevy_tweening;

use mplk_color::OutlineColor;

pub use bevy_tweening::lens::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OutlineColorLens {
    pub start: Color,
    pub end: Color,
}
impl Lens<OutlineColor> for OutlineColorLens {
    fn lerp(&mut self, mut target: Mut<OutlineColor>, ratio: f32) {
        target.0 = self.start.mix(&self.end, ratio);
    }
}
