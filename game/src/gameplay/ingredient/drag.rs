use bevy::window::WindowResized;

use crate::{
    gameplay::{
        drag::{DragFocus, DragPosition, IgnoreDrag},
        ingredient::Ingredient,
        inventory::InventoryIndex,
        layout::GameLayout,
        shaker::Shaker,
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            move_ingredients_on_resize,
            //     drag_ingredient
        )
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_observer(on_drop_enter::<Shaker>)
        .add_observer(on_drop_leave::<Shaker>);
}

fn on_drop_enter<TDropTarget: Component>(
    ev: On<PointerOver>,
    mut ingredient_q: Query<&mut DragPosition, (With<Ingredient>, With<DragFocus>)>,
    droppable_q: Query<&GlobalTransform, With<TDropTarget>>,
) {
    let drop_t = or_return_quiet!(droppable_q.get(ev.entity));
    let mut drag = or_return_quiet!(ingredient_q.single_mut());
    drag.snapped_position = Some(drop_t.translation().truncate());
}

fn on_drop_leave<TDropTarget: Component>(
    ev: On<PointerLeave>,
    mut ingredient_q: Query<&mut DragPosition, (With<Ingredient>, With<DragFocus>)>,
    droppable_q: Query<(), With<TDropTarget>>,
) {
    or_return_quiet!(droppable_q.get(ev.entity));
    let mut drag = or_return_quiet!(ingredient_q.single_mut());
    drag.snapped_position = None;
}

// fn snap_pieces_to_grid(
//     mut piece_q: Query<(&Piece, &mut DragPosition), With<DragFocus>>,
//     grid: Single<&IngredientGrid>,
// ) {
//     for (piece, mut drag) in &mut piece_q {
//         if let Some(tile) = grid.world_to_tile(drag.drag_position)
//             && let Ok(_completed_sections) = grid.preview_piece_placement(piece, tile)
//         {
//             let tile_pos = or_continue!(grid.tile_to_world(tile));
//             drag.snapped_position = Some(tile_pos);
//         } else {
//             drag.snapped_position = None;
//         }
//     }
// }

// todo: on shaker drop, piece drop, grid drop, capy drop, customer drop etc.
// pub(super) fn on_drop(
//     ev: On<DraggableDrop<Piece, IngredientGrid>>,
//     mut grid: Single<&mut IngredientGrid>,
//     mut section_mw: MessageWriter<GridSectionCompletedMsg>,
// ) {
//     let tile = or_return!(grid.world_to_tile_center(ev.position));
//     let completed_sections = or_return!(grid.place_piece(&ev.draggable, tile));
//     tracing::warn!(?completed_sections);
//     for cs in completed_sections {
//         section_mw.write(GridSectionCompletedMsg(cs));
//     }
// }

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
