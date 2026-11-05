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
