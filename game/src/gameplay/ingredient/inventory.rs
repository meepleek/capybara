use bevy::window::WindowResized;

use crate::{
    gameplay::{
        drag::IgnoreDrag, grid::GridSectionCompletedMsg, ingredient::Ingredient,
        inventory::InventoryIndex, layout::GameLayout,
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (add_ingredient, move_ingredients_on_resize).run_if(in_state(Screen::Gameplay)),
    );
}

pub(crate) const INGREDIENT_MAX: u8 = 6;

fn add_ingredient(
    mut section_mr: MessageReader<GridSectionCompletedMsg>,
    ingredient_q: Query<(), With<Ingredient>>,
    mut cmd: Commands,
    layout: GameLayout,
) {
    let mut idx = ingredient_q.count() as u8;
    for msg in section_mr.read() {
        if idx >= INGREDIENT_MAX {
            tracing::info!(count = idx, "at max ingredients");
            return;
        }

        let ingredient = Ingredient::section_ingredient(msg.0);
        let position = layout.ingredient_position(idx);
        cmd.spawn_scene(bsn! {
            @{ingredient.icon_scene(position)}
            ingredient
            InventoryIndex(idx)
            DespawnOnExit<Screen>(Screen::Gameplay)
        });
        idx += 1;
    }
}

// todo: need to handle returning ingredients as well
fn move_ingredients_on_resize(
    mut resize_mr: MessageReader<WindowResized>,
    mut ingredient_q: Query<
        (&mut Transform, &InventoryIndex),
        (With<Ingredient>, Without<IgnoreDrag>),
    >,
    layout: GameLayout,
) {
    if resize_mr.clear_any() {
        for (mut t, idx) in &mut ingredient_q {
            t.translation = layout.ingredient_position(idx.0).extend(t.translation.z);
        }
    }
}
