use mplk_grid::bit_grid::BitGrid;

use crate::gameplay::ingredient::grid::IngredientGrid;
pub use crate::prelude::*;

pub(crate) mod grid;
pub(crate) mod piece;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin, piece::plugin));
    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

pub(crate) const INGREDIENT_TILE_SIZE: u16 = 72;
pub(crate) const INGREDIENT_TILE_SIZE_F32: f32 = INGREDIENT_TILE_SIZE as _;

fn setup(mut cmd: Commands) {
    let grid = BitGrid::new((9, 9), INGREDIENT_TILE_SIZE);
    let t = Transform::from_xyz(-300., -150., 0.);
    cmd.spawn((
        IngredientGrid::new(grid, t.translation.truncate()).unwrap(),
        t,
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
