use crate::gameplay::ingredient::{grid::IngredientGrid, piece::IngredientGridPiece};
use crate::prelude::*;

use bevy::{color::palettes::tailwind, dev_tools::states::log_transitions};

#[derive(Default, PartialEq, Eq, Debug)]
pub enum HotpatchMode {
    #[default]
    Nothing,
    ResetState,
}

#[derive(Resource, Debug)]
pub struct DebugOptions {
    hotpatch_mode: HotpatchMode,
    show_debug_ui: bool,
}
impl Default for DebugOptions {
    fn default() -> Self {
        Self {
            hotpatch_mode: default(),
            show_debug_ui: true,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DebugOptions>();
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(KeyCode::KeyU)),
    );
    app.add_systems(
        Update,
        toggle_hotpatch_mode.run_if(input_just_pressed(KeyCode::KeyH)),
    );
    app.add_systems(
        Update,
        reset_scene_on_hotpatch.run_if(on_message::<bevy::ecs::HotPatched>),
    );
    app.add_systems(Update, (draw_piece_gizmos, draw_debug_ui));
    app.add_systems(Update, log_transitions::<Screen>);
}

fn toggle_hotpatch_mode(mut options: ResMut<DebugOptions>) {
    use HotpatchMode::*;

    options.hotpatch_mode = match options.hotpatch_mode {
        Nothing => ResetState,
        ResetState => Nothing,
    };
}

fn toggle_debug_ui(mut options: ResMut<DebugOptions>) {
    options.show_debug_ui = !options.show_debug_ui;
}

fn reset_scene_on_hotpatch(
    mut next: ResMut<NextState<Screen>>,
    state: Res<State<Screen>>,
    options: Res<DebugOptions>,
) {
    if options.hotpatch_mode == HotpatchMode::ResetState {
        next.set(*state.get());
    }
}

fn draw_debug_ui(mut gizmos: Gizmos, options: Res<DebugOptions>, window: Single<&Window>) {
    let win_half_size = window.size() / 2.;
    let col = tailwind::INDIGO_400;
    gizmos.text_2d(
        Isometry2d::from_translation(win_half_size + Vec2::splat(-20.)),
        &format!(
            "hp_mode(h): {:?}\nui(u): {}",
            options.hotpatch_mode, options.show_debug_ui
        ),
        12.,
        Vec2::splat(0.5),
        col,
    );
    let rect_size = Vec2::new(230., 60.);
    gizmos.rounded_rect_2d(
        Isometry2d::from_translation(win_half_size - rect_size / 2. + Vec2::splat(-10.)),
        rect_size,
        col,
    );
}

fn draw_piece_gizmos(
    grid: Single<&IngredientGrid>,
    piece_q: Query<(&GlobalTransform, &IngredientGridPiece)>,
    mut gizmos: Gizmos,
    options: Res<DebugOptions>,
) {
    if !options.show_debug_ui {
        return;
    }

    let floaty_tile_size = grid.tile_size() as f32;
    for (piece_t, piece) in &piece_q {
        for t in &piece.tiles {
            gizmos.rounded_rect_2d(
                Isometry2d::from_translation(
                    piece_t.translation().truncate() + t.as_vec2() * floaty_tile_size,
                ),
                Vec2::splat(floaty_tile_size),
                tailwind::AMBER_600,
            );
        }
        gizmos.cross_2d(
            Isometry2d::from_translation(
                piece_t.translation().truncate() + piece.pivot.as_vec2() * floaty_tile_size,
            ),
            floaty_tile_size * 0.35,
            tailwind::AMBER_800,
        );
    }
}
