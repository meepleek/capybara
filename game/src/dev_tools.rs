use crate::gameplay::grid::IngredientGrid;
use crate::gameplay::ingredient::{AddIngredient, CommonIngredient, Ingredient};
use crate::prelude::*;

use bevy::dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin};
use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::{color::palettes::tailwind, dev_tools::states::log_transitions};
use mplk_grid::debug::DebugGridPlugin;

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
    app.add_plugins((
        DebugPickingPlugin,
        DebugGridPlugin::<IngredientGrid, ()>::default(),
    ));
    app.init_resource::<DebugOptions>();
    app.insert_resource(DebugPickingMode::Normal);
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
    app.add_systems(Update, (draw_debug_ui, add_ingredient));
    app.add_systems(Update, log_transitions::<Screen>);
}

fn toggle_hotpatch_mode(mut options: ResMut<DebugOptions>) {
    use HotpatchMode::*;

    options.hotpatch_mode = match options.hotpatch_mode {
        Nothing => ResetState,
        ResetState => Nothing,
    };
}

fn toggle_debug_ui(mut options: ResMut<DebugOptions>, mut picking_mode: ResMut<DebugPickingMode>) {
    if options.show_debug_ui {
        options.show_debug_ui = false;
        *picking_mode = DebugPickingMode::Disabled;
    } else {
        options.show_debug_ui = true;
        *picking_mode = DebugPickingMode::Normal;
    }
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

fn add_ingredient(mut kb_input_mr: MessageReader<KeyboardInput>, mut cmd: Commands) {
    use CommonIngredient::*;

    for ev in kb_input_mr.read() {
        if !ev.state.is_pressed() || ev.repeat {
            continue;
        }

        let ingredient = match ev.key_code {
            KeyCode::Digit0 => Water,
            KeyCode::Digit1 => Apple,
            KeyCode::Digit2 => Orange,
            KeyCode::Digit3 => Ginger,
            KeyCode::Digit4 => Honey,
            KeyCode::Digit5 => Air,
            KeyCode::Digit6 => Coffee,
            _ => continue,
        };
        cmd.trigger(AddIngredient(Ingredient::Common(ingredient)));
    }
}
