use bevy::{ecs::system::SystemParam, window::WindowResized};

use crate::{
    gameplay::{
        cauldron::Cauldron,
        drag::IgnoreDrag,
        grid::{GridSection, IngredientGrid},
        ingredient::Ingredient,
        inventory::InventoryIndex,
        piece::Piece,
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            move_inventory_on_resize::<Ingredient>,
            move_inventory_on_resize::<Piece>,
            move_element_on_resize::<IngredientGrid>,
            move_element_on_resize::<Cauldron>,
        )
            .run_if(in_state(Screen::Gameplay)),
    );
}

pub trait InventoryPosition {
    fn inventory_position(layout: &GameLayout, idx: &InventoryIndex) -> Vec2;
}

pub trait LayoutElementPosition {
    fn position(layout: &GameLayout, grid: &IngredientGrid) -> Vec2;
}

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

    pub fn grid_section_position(&self, grid: &IngredientGrid, section: GridSection) -> Vec2 {
        let grid_half = grid.world_size() / 2.;
        let grid_ninth = grid.world_size() / 9.;
        let grid_section_center_offset = grid_ninth / 2.;
        let grid_section_padding = grid.tile_size() as f32 * 0.7;
        let tile_size = grid.tile_size() as f32;

        match section {
            GridSection::Column(col) => {
                grid_half * Vec2::new(-1., 1.)
                    + col as f32 * grid_ninth * Vec2::X
                    + grid_section_center_offset * Vec2::X
                    + grid_section_padding * Vec2::Y
            }
            GridSection::Row(row) => {
                grid_half - row as f32 * grid_ninth * Vec2::Y - grid_section_center_offset * Vec2::Y
                    + grid_section_padding * Vec2::X
            }
            GridSection::Area(idx) => {
                let tile = grid.area_idx_to_start_tile(idx).expect("invalid area idx");
                grid_half * Vec2::new(-1., 1.)
                    + tile.as_world_vec2() * tile_size
                    + tile_size * TileCoords::ONE.as_world_vec2() * 1.5
            }
        }
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

    pub fn ingredient_position(&self, idx: u8) -> Vec2 {
        let window_half = self.window_half_size();
        Vec2::new(
            window_half.x // right edge
                - 20. // extra margin
                - 90. * idx as f32 // piece idx offset
                - Self::PADDING,
            window_half.y - 40. - Self::PADDING,
        )
    }

    pub fn shaker_position(&self) -> Vec2 {
        let window_half = self.window_half_size();
        Vec2::new(
            window_half.x // right edge
                - 90. // extra margin
                - Self::PADDING,
            0.,
        )
    }

    fn window_half_size(&self) -> Vec2 {
        self.window.size() / 2.
    }
}

// todo: need to handle returning inventory items as well
fn move_inventory_on_resize<TInventoryItem: Component + InventoryPosition>(
    mut resize_mr: MessageReader<WindowResized>,
    mut ingredient_q: Query<
        (&mut Transform, &InventoryIndex),
        (With<TInventoryItem>, Without<IgnoreDrag>),
    >,
    layout: GameLayout,
) {
    if resize_mr.clear_any() {
        for (mut t, idx) in &mut ingredient_q {
            t.translation =
                TInventoryItem::inventory_position(&layout, idx).extend(t.translation.z);
        }
    }
}

fn move_element_on_resize<TElement: Component + LayoutElementPosition>(
    mut resize_mr: MessageReader<WindowResized>,
    mut transform_q: Query<&mut Transform, With<TElement>>,
    grid: Single<&IngredientGrid>,
    layout: GameLayout,
) {
    if resize_mr.clear_any() {
        for mut t in &mut transform_q {
            t.translation = TElement::position(&layout, &grid).extend(t.translation.z);
        }
    }
}
