use crate::{
    gameplay::{
        drag::{DragFocus, DragPosition, DraggableDrop, DraggableSnapChange, SnapData},
        grid::{GridSectionCompletedMsg, IngredientGrid},
        piece::Piece,
    },
    prelude::*,
    screens::run_conditions::in_game,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, snap_pieces_to_grid.run_if(in_game));
    app.add_observer(on_drop);
}

pub(super) fn on_drop(
    ev: On<DraggableDrop<Piece, IngredientGrid>>,
    mut grid: Single<&mut IngredientGrid>,
    mut section_mw: MessageWriter<GridSectionCompletedMsg>,
) {
    let center_offset = ev.draggable.center * grid.tile_size() as f32;
    let position = ev.position + center_offset;
    let tile = or_return!(grid.world_to_tile_center(position));
    let completed_sections = or_return!(grid.place_piece(&ev.draggable, tile));
    tracing::info!(?completed_sections);
    for cs in completed_sections {
        section_mw.write(GridSectionCompletedMsg(cs));
    }
}

fn snap_pieces_to_grid(
    mut piece_q: Query<(Entity, &Piece, &mut DragPosition, &DragFocus)>,
    grid: Single<(Entity, &IngredientGrid)>,
    mut cmd: Commands,
) {
    let (grid_e, grid) = *grid;
    let tile_size = grid.tile_size() as f32;
    for (piece_e, piece, mut drag, focus) in &mut piece_q {
        if *focus != DragFocus::Active {
            continue;
        }

        let center_offset = piece.center * tile_size;
        let centered_position = drag.drag_position + center_offset;
        if let Some(tile) = grid.world_to_tile(centered_position)
            && let Ok(_completed_sections) = grid.preview_piece_placement(piece, tile)
        {
            let tile_pos = or_continue!(grid.tile_to_world(tile)) - center_offset;
            drag.snapped_data = Some(SnapData {
                position: tile_pos,
                entity: grid_e,
            });
            cmd.try_trigger(grid_e, |e| {
                DraggableSnapChange::<Piece, IngredientGrid>::snapped(piece_e, e, tile_pos)
            });
        } else {
            drag.snapped_data = None;
            cmd.try_trigger(grid_e, |e| {
                DraggableSnapChange::<Piece, IngredientGrid>::unsnapped(piece_e, e)
            });
        }
    }
}
