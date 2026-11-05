use crate::{
    gameplay::{
        drag::DragPosition,
        grid::GridSectionCompletedMsg,
        ingredient::{INGREDIENT_MAX, Ingredient},
        inventory::InventoryIndex,
        layout::GameLayout,
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, add_ingredient.run_if(in_state(Screen::Gameplay)));
}

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
            DragPosition::new(position)
            ingredient
            InventoryIndex(idx)
            DespawnOnExit<Screen>(Screen::Gameplay)
            Pickable
            on(gameplay::drag::on_draggable_hover)
            on(gameplay::drag::on_draggable_blur)
            on(gameplay::drag::on_draggable_press)
            on(gameplay::drag::on_draggable_drag)
            // on(piece::drag::on_drag_end)
            Children[
                #piece_picking_overlay
                Mesh2d::from_collection(SpriteAssets::piece_picking_mesh)
                Pickable
            ]
        });
        idx += 1;
    }
}
