use crate::{
    gameplay::{
        cauldron::Cauldron,
        drag::{DragFocus, DragPosition, SnapData},
        ingredient::Ingredient,
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_drop_enter::<Cauldron>)
        .add_observer(on_drop_leave::<Cauldron>);
}

fn on_drop_enter<TDropTarget: Component>(
    ev: On<PointerOver>,
    mut ingredient_q: Query<&mut DragPosition, (With<Ingredient>, With<DragFocus>)>,
    target_q: Query<(Entity, &GlobalTransform), With<TDropTarget>>,
) {
    let (target_e, drop_t) = or_return_quiet!(target_q.get(ev.entity));
    let mut drag = or_return_quiet!(ingredient_q.single_mut());
    drag.snapped_data = Some(SnapData {
        position: drop_t.translation().truncate(),
        entity: target_e,
    });
}

fn on_drop_leave<TDropTarget: Component>(
    ev: On<PointerLeave>,
    mut ingredient_q: Query<&mut DragPosition, (With<Ingredient>, With<DragFocus>)>,
    droppable_q: Query<(), With<TDropTarget>>,
) {
    or_return_quiet!(droppable_q.get(ev.entity));
    let mut drag = or_return_quiet!(ingredient_q.single_mut());
    drag.snapped_data = None;
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

// todo: on cauldron drop, piece drop, grid drop, capy drop, customer drop etc.
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
