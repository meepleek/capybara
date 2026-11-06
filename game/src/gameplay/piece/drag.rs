use crate::{
    gameplay::{
        drag::{DragFocus, DragPosition, DraggableDrop},
        grid::{GridSectionCompletedMsg, IngredientGrid},
        piece::Piece,
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        snap_pieces_to_grid.run_if(in_state(Screen::Gameplay)),
    );
}

pub(super) fn on_drop(
    ev: On<DraggableDrop<Piece, IngredientGrid>>,
    mut grid: Single<&mut IngredientGrid>,
    mut section_mw: MessageWriter<GridSectionCompletedMsg>,
) {
    let tile = or_return!(grid.world_to_tile_center(ev.position));
    let completed_sections = or_return!(grid.place_piece(&ev.draggable, tile));
    tracing::warn!(?completed_sections);
    for cs in completed_sections {
        section_mw.write(GridSectionCompletedMsg(cs));
    }
}

fn snap_pieces_to_grid(
    mut piece_q: Query<(&Piece, &mut DragPosition), With<DragFocus>>,
    grid: Single<&IngredientGrid>,
) {
    for (piece, mut drag) in &mut piece_q {
        if let Some(tile) = grid.world_to_tile(drag.drag_position)
            && let Ok(_completed_sections) = grid.preview_piece_placement(piece, tile)
        {
            let tile_pos = or_continue!(grid.tile_to_world(tile));
            drag.snapped_position = Some(tile_pos);
        } else {
            drag.snapped_position = None;
        }
    }
}
