use mplk_ecs::transform::TrackPosition;
use mplk_grid::bit_grid::BitGrid;

use crate::gameplay::{
    ingredient::grid::{GridSection, IngredientGrid},
    layout::GameLayout,
};
pub use crate::prelude::*;

pub(crate) mod grid;
pub(crate) mod piece;
pub(crate) mod piece_template;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin, piece::plugin));
    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

pub(crate) const INGREDIENT_GRID_SIZE: u16 = 9;
pub(crate) const INGREDIENT_TILE_SIZE: u16 = 64;
pub(crate) const INGREDIENT_TILE_SIZE_F32: f32 = INGREDIENT_TILE_SIZE as _;

#[derive(Debug)]
pub enum Ingredient {
    Honey,
    HoneyPlus,
    Apple,
    Pineapple,
    Ginger,
    Jalapeno,
    Orange,
    Lime,
    Air,
    AirPlus,
    Coffee,
    Chocolate,
    Water,
    Ice,
}
impl Ingredient {
    pub fn spritesheet_idx(&self) -> usize {
        match self {
            Ingredient::Honey => 8,
            Ingredient::HoneyPlus => 16,
            Ingredient::Apple => 9,
            Ingredient::Pineapple => 17,
            Ingredient::Ginger => 10,
            Ingredient::Jalapeno => 18,
            Ingredient::Orange => 11,
            Ingredient::Lime => 19,
            Ingredient::Air => 12,
            Ingredient::AirPlus => 20,
            Ingredient::Coffee => 13,
            Ingredient::Chocolate => 21,
            Ingredient::Water => 14,
            Ingredient::Ice => 22,
        }
    }

    pub fn section_ingredient(section: GridSection) -> Ingredient {
        use GridSection::*;
        use Ingredient::*;

        match section {
            Column(0..=2) | Row(6..=8) | Area(1) => Apple,
            Column(3..=5) | Row(3..=5) => Orange,
            Column(6..=8) | Row(0..=2) => Honey,
            Area(0) | Area(8) => Coffee,
            Area(2) | Area(4) | Area(6) => Water,
            Area(3) | Area(5) => Air,
            Area(7) => Ginger,
            _ => unreachable!("invalid grid section"),
        }
    }
}

fn setup(mut cmd: Commands, layout: GameLayout) {
    let grid = BitGrid::new(
        (INGREDIENT_GRID_SIZE, INGREDIENT_GRID_SIZE),
        INGREDIENT_TILE_SIZE,
    );
    let mut grid = IngredientGrid::new(grid, Vec2::ZERO).unwrap();
    let position = layout.grid_position(&grid);
    grid.set_position(position);

    let mut ingredients: Vec<Box<dyn Scene>> = Vec::new();
    let mut sections: Vec<_> = [1, 4, 7]
        .into_iter()
        .map(|i| [GridSection::Row(i), GridSection::Column(i)])
        .flatten()
        .collect();
    sections.extend((0..9).map(GridSection::Area));

    for section in sections {
        let ingredient = Ingredient::section_ingredient(section);
        let position = layout.grid_section_position(&grid, section);
        let index = ingredient.spritesheet_idx();
        let ingredient_scene = bsn! {
            #ingredient_src_icon
            Transform { translation: {position.extend(0.)} }
            from_collection(move |sprites: &SpriteAssets| Sprite {
                image: sprites.sheet.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: sprites.layout.clone(),
                    index
                }),
                custom_size: Some(Vec2::splat(INGREDIENT_TILE_SIZE_F32)),
                ..default()
            })
        };
        ingredients.push(Box::new(ingredient_scene));
    }

    cmd.spawn_scene(bsn! {
        grid
        Transform::from_translation(position.extend(0.))
        Visibility
        DespawnOnExit<Screen>(Screen::Gameplay)
        Children[{ingredients}]
    });
}

// fn track_mouse(
//     cam: Single<(&GlobalTransform, &Camera), With<PrimaryCamera>>,
//     window: Single<&Window>,
//     mut grid_q: Query<&mut IngredientGrid>,
// ) {
//     let (cam_t, cam) = *cam;
//     if let Some(cursor_pos) = window.cursor_position()
//         && let Ok(cursor_world_pos) = cam.viewport_to_world_2d(cam_t, cursor_pos)
//     {
//         for mut g in &mut grid_q {
//             if let Some(tile) = g.world_to_tile(cursor_world_pos) {
//                 g.place_tiles([tile]).unwrap();
//             }
//         }
//     }
// }
