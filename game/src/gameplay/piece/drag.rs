use bevy::window::WindowResized;
use mplk_utils::math::asymptotic_smoothing_with_delta_time;

use crate::{
    gameplay::{
        drag::{DragFocus, DragPosition, IgnoreDrag},
        grid::{GridSectionCompletedMsg, IngredientGrid},
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

pub(super) fn on_drag_end(
    ev: On<PointerDragEnd>,
    mut piece_q: Query<
        (&mut Transform, &Piece, &mut DragPosition, &InventoryIndex),
        Without<IgnoreDrag>,
    >,
    mut grid: Single<&mut IngredientGrid>,
    mut cmd: Commands,
    layout: GameLayout,
    mut section_mw: MessageWriter<GridSectionCompletedMsg>,
) {
    let (mut t, piece, mut drag, piece_idx) = or_return!(piece_q.get_mut(ev.entity));
    if let Some(snapped_pos) = drag.snapped_position
        && let Some(tile) = grid.world_to_tile(snapped_pos)
    {
        let completed_sections = or_return!(grid.place_piece(piece, tile));
        tracing::warn!(?completed_sections);
        for cs in completed_sections {
            section_mw.write(GridSectionCompletedMsg(cs));
        }
        // set position in case it's still easing
        t.translation = snapped_pos.extend(t.translation.z);
        cmd.try_remove_from::<Piece>(ev.entity);
        cmd.try_insert_to(
            ev.entity,
            TransformScaleLensSrc::new(Vec2::ZERO)
                .duration(ms(160))
                .despawn_target_on_completion(),
        );
    } else {
        cmd.try_remove_from::<DragFocus>(ev.entity);
        let start_pos = layout.piece_position(piece_idx.0);
        drag.drag_position = start_pos;
        let tween_duration = ms(330);
        cmd.try_insert_to(
            ev.entity,
            (
                TransformPositionLensSrc::new(start_pos)
                    .duration(tween_duration)
                    .easing(EaseFunction::BackOut),
                IgnoreDrag,
            ),
        );
        cmd.delayed()
            .duration(tween_duration)
            .entity(ev.entity)
            .try_remove::<IgnoreDrag>();
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
    mut piece_q: Query<(&mut Transform, &Piece, &mut DragPosition), With<DragFocus>>,
    grid: Single<&IngredientGrid>,
    time: Res<Time>,
) {
    for (mut t, piece, mut drag) in &mut piece_q {
        if let Some(tile) = grid.world_to_tile(drag.drag_position)
            && let Ok(_completed_sections) = grid.preview_piece_placement(piece, tile)
        {
            let tile_pos = or_continue!(grid.tile_to_world(tile));
            drag.snapped_position = Some(tile_pos);
        } else {
            drag.snapped_position = None;
        }

        let target = drag.snapped_position.unwrap_or(drag.drag_position);
        t.translation = asymptotic_smoothing_with_delta_time(
            t.translation,
            target.extend(t.translation.z),
            0.4,
            time.delta_secs(),
        );
    }
}
