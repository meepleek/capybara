use bevy::window::WindowResized;
use mplk_utils::math::asymptotic_smoothing_with_delta_time;

use crate::{
    gameplay::{
        drag::{DragFocus, DragPosition, IgnoreDrag},
        ingredient::Ingredient,
        inventory::InventoryIndex,
        layout::GameLayout,
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
}

// fn drag_ingredient(
//     mut piece_q: Query<(&mut Transform, &DragPosition), (With<DragFocus>, With<Ingredient>)>,
//     time: Res<Time>,
// ) {
//     for (mut t, drag) in &mut piece_q {
//         let target = drag.snapped_position.unwrap_or(drag.drag_position);
//         t.translation = asymptotic_smoothing_with_delta_time(
//             t.translation,
//             target.extend(t.translation.z),
//             0.4,
//             time.delta_secs(),
//         );
//     }
// }

// todo: drop handling
// pub(super) fn on_drop_enter<
//     TDraggable: Component + InventoryPosition + Clone,
//     TDropTarget: Component,
// >(
//     ev: On<PointerDragEnter>,
//     mut draggable_q: Query<&mut DragPosition, With<TDraggable>>,
//     droppable_q: Query<&GlobalTransform, With<TDropTarget>>,
//     // mut piece_q: Query<
//     //     (&mut Transform, &Piece, &mut DragPosition, &InventoryIndex),
//     //     Without<IgnoreDrag>,
//     // >,
//     // mut cmd: Commands,
//     // layout: GameLayout,
// ) {
//     let mut drag = or_return_quiet!(draggable_q.get_mut(ev.dragged));
//     let drop_t = or_return_quiet!(droppable_q.get(ev.entity));

//     drag.snapped_position = Some(drop_t.translation().truncate());
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
