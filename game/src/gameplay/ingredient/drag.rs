use crate::{
    gameplay::{
        cauldron::Cauldron,
        drag::{DragFocus, DragPosition, DraggableSnapChange, SnapData, ValidDragTarget},
        ingredient::Ingredient,
        piece::Piece,
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_drop_enter::<Cauldron>)
        .add_observer(on_drop_leave::<Cauldron>)
        .add_observer(on_drop_enter::<Piece>)
        .add_observer(on_drop_leave::<Piece>);
}

fn on_drop_enter<TDragTarget: Component + ValidDragTarget<Ingredient>>(
    ev: On<PointerOver>,
    mut ingredient_q: Query<(Entity, &mut DragPosition, &Ingredient), With<DragFocus>>,
    target_q: Query<(Entity, &GlobalTransform), With<TDragTarget>>,
    mut cmd: Commands,
) {
    let (target_e, drop_t) = or_return_quiet!(target_q.get(ev.entity));
    let (ingredient_e, mut drag, ingredient) = or_return_quiet!(ingredient_q.single_mut());
    if !TDragTarget::valid(ingredient) {
        return;
    }

    let position = drop_t.translation().truncate();
    drag.snapped_data = Some(SnapData {
        position,
        entity: target_e,
    });
    cmd.try_trigger(target_e, |e| {
        DraggableSnapChange::<Ingredient, TDragTarget>::snapped(ingredient_e, e, position)
    });
}

fn on_drop_leave<TDragTarget: Component + ValidDragTarget<Ingredient>>(
    ev: On<PointerLeave>,
    mut ingredient_q: Query<(Entity, &mut DragPosition, &Ingredient), With<DragFocus>>,
    target_q: Query<(), With<TDragTarget>>,
    mut cmd: Commands,
) {
    or_return_quiet!(target_q.get(ev.entity));
    let (ingredient_e, mut drag, ingredient) = or_return_quiet!(ingredient_q.single_mut());
    if !TDragTarget::valid(ingredient) {
        return;
    }

    drag.snapped_data = None;
    cmd.try_trigger(ev.entity, |e| {
        DraggableSnapChange::<Ingredient, TDragTarget>::unsnapped(ingredient_e, e)
    });
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
