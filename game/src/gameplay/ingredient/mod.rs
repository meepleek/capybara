use mplk_ecs::transform::TrackPosition;
use mplk_grid::bit_grid::BitGrid;

use crate::gameplay::{ingredient::grid::IngredientGrid, layout::GameLayout};
pub use crate::prelude::*;

pub(crate) mod grid;
pub(crate) mod piece;
pub(crate) mod piece_template;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin, piece::plugin));
    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

pub(crate) const INGREDIENT_GRID_SIZE: u16 = 9;
pub(crate) const INGREDIENT_TILE_SIZE: u16 = 72;
pub(crate) const INGREDIENT_TILE_SIZE_F32: f32 = INGREDIENT_TILE_SIZE as _;

fn setup(mut cmd: Commands, layout: GameLayout) {
    let grid = BitGrid::new(
        (INGREDIENT_GRID_SIZE, INGREDIENT_GRID_SIZE),
        INGREDIENT_TILE_SIZE,
    );
    let mut grid = IngredientGrid::new(grid, Vec2::ZERO).unwrap();
    let position = layout.grid_position(&grid);
    grid.set_position(position);
    tracing::warn!(?position);
    cmd.spawn((
        grid,
        Transform::from_translation(position.extend(0.)),
        DespawnOnExit(Screen::Gameplay),
    ));
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
