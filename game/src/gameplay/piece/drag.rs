use bevy::window::WindowResized;
use mplk_utils::math::asymptotic_smoothing_with_delta_time;

use crate::{
    gameplay::{
        drag::{DragFocus, DragPosition, DraggableDrop, IgnoreDrag},
        grid::{GridSectionCompletedMsg, GridTile, IngredientGrid},
        inventory::InventoryIndex,
        layout::GameLayout,
        piece::Piece,
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (snap_pieces_to_grid, move_pieces_on_resize).run_if(in_state(Screen::Gameplay)),
    );
}

pub(super) fn on_drop(
    ev: On<DraggableDrop<Piece, GridTile>>,
    mut grid: Single<&mut IngredientGrid>,
    tile_q: Query<&GridTile>,
    mut section_mw: MessageWriter<GridSectionCompletedMsg>,
) {
    tracing::warn!(?ev, "piece drop");
    let tile = or_return!(tile_q.get(ev.drop_entity));
    let completed_sections = or_return!(grid.place_piece(&ev.draggable, tile.0));
    tracing::warn!(?completed_sections);
    for cs in completed_sections {
        section_mw.write(GridSectionCompletedMsg(cs));
    }
}

// todo: need to handle returning ingredients as well
fn move_pieces_on_resize(
    mut resize_mr: MessageReader<WindowResized>,
    mut ingredient_q: Query<
        (&mut Transform, &mut DragPosition, &InventoryIndex),
        (With<Piece>, Without<IgnoreDrag>),
    >,
    layout: GameLayout,
) {
    if resize_mr.clear_any() {
        for (mut t, mut drag, idx) in &mut ingredient_q {
            let position = layout.piece_position(idx.0);
            t.translation = position.extend(t.translation.z);
            drag.drag_position = position;
        }
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
