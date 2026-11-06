use mplk_ecs::transform::TrackPosition;
use mplk_grid::bit_grid::BitGrid;

use crate::gameplay::{
    grid::{GridSection, GridTile, INGREDIENT_GRID_SIZE, INGREDIENT_TILE_SIZE, IngredientGrid},
    ingredient::Ingredient,
    layout::GameLayout,
};
pub use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

fn setup(mut cmd: Commands, layout: GameLayout) {
    let grid = BitGrid::new(
        (INGREDIENT_GRID_SIZE, INGREDIENT_GRID_SIZE),
        INGREDIENT_TILE_SIZE,
    );
    let mut grid = IngredientGrid::new(grid, Vec2::ZERO).unwrap();
    let position = layout.grid_position(&grid);
    grid.set_position(position);

    let mut children: Vec<Box<dyn Scene>> = Vec::new();
    let mut sections: Vec<_> = [1, 4, 7]
        .into_iter()
        .map(|i| [GridSection::Row(i), GridSection::Column(i)])
        .flatten()
        .collect();
    sections.extend((0..9).map(GridSection::Area));

    for section in sections {
        let ingredient = Ingredient::section_ingredient(section);
        let position = layout.grid_section_position(&grid, section);
        let ingredient_scene = ingredient.icon_scene(position);
        children.push(Box::new(ingredient_scene));
    }

    for tile in grid.iter_tiles() {
        let tile_scene = bsn! {
            #grid_tile
            GridTile(tile)
            Transform { translation: { (tile.as_world_vec2() * grid.tile_size() as f32).extend(0.) } }
            Mesh2d::from_collection(SpriteAssets::piece_picking_mesh)
            Pickable
        };
        children.push(Box::new(tile_scene));
    }

    cmd.spawn_scene(bsn! {
        grid
        Transform::from_translation(position.extend(0.))
        Visibility
        DespawnOnExit<Screen>(Screen::Gameplay)
        Children[{children}]
    });
}
