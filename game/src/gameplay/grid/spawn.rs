use mplk_ecs::transform::TrackPosition;
use mplk_grid::bit_grid::BitGrid;

use crate::gameplay::{
    grid::{GridSection, INGREDIENT_GRID_SIZE, INGREDIENT_TILE_SIZE, IngredientGrid},
    ingredient::CommonIngredient,
    layout::GameLayout,
};
pub use crate::prelude::*;

pub(super) fn plugin(_app: &mut App) {}

pub fn grid_scene(layout: &GameLayout, grid: Option<BitGrid>, level_idx: u8) -> impl Scene {
    let grid = grid.unwrap_or_else(|| {
        BitGrid::new(
            (INGREDIENT_GRID_SIZE, INGREDIENT_GRID_SIZE),
            INGREDIENT_TILE_SIZE,
        )
    });
    let mut grid = IngredientGrid::new(grid, Vec2::ZERO).expect("invalid bitgrid");
    let position = layout.grid_position(&grid);
    grid.set_position(position);

    let mut ingredient_icons: Vec<Box<dyn Scene>> = Vec::new();
    let mut sections: Vec<_> = [1, 4, 7]
        .into_iter()
        .map(|i| [GridSection::Row(i), GridSection::Column(i)])
        .flatten()
        .collect();
    sections.extend((0..9).map(GridSection::Area));

    for section in sections {
        let ingredient = CommonIngredient::section_ingredient(section, level_idx);
        let position = layout.grid_section_position(&grid, section);
        let ingredient_scene = ingredient.icon_scene(position.extend(0.));
        ingredient_icons.push(Box::new(ingredient_scene));
    }

    bsn! {
        grid
        Transform::from_translation(position.extend(0.))
        Visibility
        DespawnOnExit<Screen>(Screen::Gameplay)
        Children[{ingredient_icons}]
    }
}
