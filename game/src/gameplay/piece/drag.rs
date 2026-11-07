use crate::{
    gameplay::{
        drag::{DragFocus, DragPosition, DraggableDrop, SnapData},
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
    let tile = or_return!(grid.world_to_tile_center(ev.position));
    let completed_sections = or_return!(grid.place_piece(&ev.draggable, tile));
    tracing::info!(?completed_sections);
    for cs in completed_sections {
        section_mw.write(GridSectionCompletedMsg(cs));
    }
}

fn snap_pieces_to_grid(
    mut piece_q: Query<(&Piece, &mut DragPosition), With<DragFocus>>,
    grid: Single<(Entity, &IngredientGrid)>,
) {
    let (grid_e, grid) = *grid;
    for (piece, mut drag) in &mut piece_q {
        if let Some(tile) = grid.world_to_tile(drag.drag_position)
            && let Ok(_completed_sections) = grid.preview_piece_placement(piece, tile)
        {
            let tile_pos = or_continue!(grid.tile_to_world(tile));
            drag.snapped_data = Some(SnapData {
                position: tile_pos,
                entity: grid_e,
            });
        } else {
            drag.snapped_data = None;
        }
    }
}
