use bevy::ecs::system::SystemParam;

use crate::{gameplay::ingredient::grid::IngredientGrid, prelude::*};

#[derive(SystemParam)]
pub struct GameLayout<'w, 's> {
    window: Single<'w, 's, &'static Window>,
}
impl GameLayout<'_, '_> {
    pub const PADDING: f32 = 64.;

    pub fn grid_position(&self, grid: &IngredientGrid) -> Vec2 {
        let grid_half = grid.world_size() / 2.;
        let window_half = self.window_half_size();
        Vec2::new(
            -window_half.x + grid_half.x + Self::PADDING,
            -window_half.y + grid_half.y + Self::PADDING,
        )
    }

    pub fn piece_position(&self, idx: u8) -> Vec2 {
        let window_half = self.window_half_size();
        Vec2::new(
            -window_half.x // left edge
                + 60. // margin
                + 250. * idx as f32 // piece idx offset
                + Self::PADDING,
            window_half.y - 40. - Self::PADDING,
        )
    }

    fn window_half_size(&self) -> Vec2 {
        self.window.size() / 2.
    }
}
