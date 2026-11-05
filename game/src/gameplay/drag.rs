use mplk_ecs::observer::ensure_single_at_most;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(ensure_single_at_most::<DragFocus>);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragFocus {
    Hover,
    Active,
}

#[derive(Component, Debug)]
pub struct IgnoreDrag;

#[derive(Debug, Component, Clone, Default)]
#[require(Transform)]
pub struct DragPosition {
    pub drag_position: Vec2,
    pub snapped_position: Option<Vec2>,
}
impl DragPosition {
    pub fn new(drag_position: Vec2) -> Self {
        Self {
            drag_position,
            snapped_position: None,
        }
    }
}

pub(super) fn on_draggable_hover(
    ev: On<PointerEnter>,
    focus_q: Query<(), With<DragFocus>>,
    mut cmd: Commands,
) {
    if !focus_q.is_empty() {
        return;
    }
    if ev.original_event_target() == ev.entity && !focus_q.contains(ev.entity) {
        cmd.try_insert_to(ev.entity, DragFocus::Hover);
    }
}

pub(super) fn on_draggable_blur(
    ev: On<PointerLeave>,
    focus_q: Query<&DragFocus, Without<IgnoreDrag>>,
    mut cmd: Commands,
) {
    let focus = or_return!(focus_q.get(ev.entity));
    if *focus == DragFocus::Hover {
        cmd.try_remove_from::<DragFocus>(ev.entity);
    }
}

pub(super) fn on_draggable_press(
    ev: On<PointerPress>,
    mut cmd: Commands,
    ignore_q: Query<(), With<IgnoreDrag>>,
) {
    if ignore_q.contains(ev.entity) {
        return;
    }
    cmd.try_insert_to(ev.entity, DragFocus::Active);
}

pub(super) fn on_draggable_drag(
    ev: On<PointerDrag>,
    mut q: Query<&mut DragPosition>,
    focus_q: Query<(&DragFocus, Has<IgnoreDrag>)>,
) {
    let (focus, ignore_drag) = or_return!(focus_q.get(ev.entity));
    if *focus != DragFocus::Active || ignore_drag {
        return;
    }
    let mut drag = or_return!(q.get_mut(ev.entity));
    let d = ev.delta;
    drag.drag_position += Vec2::new(d.x, -d.y);
}
