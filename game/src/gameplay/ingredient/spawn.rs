use crate::{
    gameplay::{
        cauldron::Cauldron,
        drag::DragPosition,
        grid::GridSectionCompletedMsg,
        ingredient::{AddIngredient, CommonIngredient, INGREDIENT_MAX, Ingredient},
        inventory::InventoryIndex,
        layout::GameLayout,
        level::LevelIndex,
        piece::Piece,
    },
    prelude::*,
    screens::run_conditions::in_game,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, process_completed_section.run_if(in_game));
    app.add_observer(add_ingredient);
}

fn process_completed_section(
    mut section_mr: MessageReader<GridSectionCompletedMsg>,
    mut cmd: Commands,
    level_idx: Res<LevelIndex>,
) {
    for msg in section_mr.read() {
        let ingredient = CommonIngredient::section_ingredient(msg.0, level_idx.0);
        for _ in 0..2 {
            cmd.trigger(AddIngredient(Ingredient::Common(ingredient)));
        }
    }
}

fn add_ingredient(
    ev: On<AddIngredient>,
    ingredient_q: Query<(), With<Ingredient>>,
    mut cmd: Commands,
    layout: GameLayout,
) {
    let idx = ingredient_q.count() as u8;
    if idx >= INGREDIENT_MAX {
        tracing::info!(count = idx, "at max ingredients");
        return;
    }
    let ingredient = ev.0;
    let position = layout.ingredient_position(idx);
    cmd.spawn_scene(ingredient_scene(ingredient, position, idx));
}

fn ingredient_scene(ingredient: Ingredient, position: Vec2, idx: u8) -> impl Scene {
    let icon = ingredient.icon_scene(position.extend(3.));
    bsn! {
        @{icon}
        DragPosition::new(position)
        ingredient
        InventoryIndex(idx)
        DespawnOnExit<Screen>(Screen::Gameplay)
        Pickable { is_hoverable: true, should_block_lower: false }
        on(gameplay::drag::on_draggable_hover)
        on(gameplay::drag::on_draggable_blur)
        on(gameplay::drag::on_draggable_press)
        on(gameplay::drag::on_draggable_drag)
        on(gameplay::drag::on_draggable_drag_end_restore_position::<Ingredient>)
        on(gameplay::drag::on_draggable_drag_end_snapped::<Ingredient, Cauldron>)
        on(gameplay::drag::on_draggable_drag_end_snapped::<Ingredient, Piece>)
        Children[
            #piece_picking_overlay
            Mesh2d::from_collection(SpriteAssets::piece_picking_mesh)
            Pickable::IGNORE
        ]
    }
}
