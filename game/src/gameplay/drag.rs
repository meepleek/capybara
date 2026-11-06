use mplk_ecs::observer::ensure_single_at_most;
use mplk_utils::math::asymptotic_smoothing_with_delta_time;

use crate::{
    gameplay::{
        inventory::InventoryIndex,
        layout::{GameLayout, InventoryPosition},
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(ensure_single_at_most::<DragFocus>);
    app.add_systems(Update, raise_z_on_drag_active)
        .add_observer(restore_z_on_drag_rmv);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragFocus {
    Hover,
    Active,
}

#[derive(Component, Debug)]
pub struct IgnoreDrag;

#[derive(Component, Debug)]
pub struct InitialTranslationZ(f32);

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

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DropTarget<T>(PhantomData<T>);

#[derive(EntityEvent, Debug, PartialEq)]
pub struct DraggableDrop<TDraggable: Component + InventoryPosition + Clone, TDropTarget: Component>
{
    pub position: Vec2,
    #[event_target]
    pub drop_entity: Entity,
    pub draggable: TDraggable,
    _drop_phantom: PhantomData<TDropTarget>,
}
impl<TDraggable: Component + InventoryPosition + Clone, TDropTarget: Component>
    DraggableDrop<TDraggable, TDropTarget>
{
    pub fn new(draggable: TDraggable, drop_entity: Entity, position: Vec2) -> Self {
        Self {
            position,
            drop_entity,
            draggable,
            _drop_phantom: default(),
        }
    }
}

fn raise_z_on_drag_active(
    mut trans_q: Query<
        (Entity, &mut Transform, &DragFocus),
        (Changed<DragFocus>, Without<InitialTranslationZ>),
    >,
    mut cmd: Commands,
) {
    for (e, mut t, focus) in &mut trans_q {
        if focus != &DragFocus::Active {
            return;
        }
        cmd.try_insert_to(e, InitialTranslationZ(t.translation.z));
        t.translation.z += 100.;
    }
}

fn restore_z_on_drag_rmv(
    ev: On<Remove<DragFocus>>,
    mut trans_q: Query<(&mut Transform, &InitialTranslationZ)>,
    mut cmd: Commands,
) {
    let (mut t, z) = or_return!(trans_q.get_mut(ev.entity));
    t.translation.z = z.0;
    cmd.try_remove_from::<InitialTranslationZ>(ev.entity);
}

pub(super) fn on_draggable_hover(
    ev: On<PointerOver>,
    focus_q: Query<(), With<DragFocus>>,
    mut cmd: Commands,
) {
    if !focus_q.is_empty() {
        return;
    }
    cmd.try_insert_to(ev.entity, DragFocus::Hover);
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
    mut q: Query<(&mut DragPosition, &mut Transform)>,
    focus_q: Query<(&DragFocus, Has<IgnoreDrag>)>,
    time: Res<Time>,
) {
    let (focus, ignore_drag) = or_return!(focus_q.get(ev.entity));
    if *focus != DragFocus::Active || ignore_drag {
        return;
    }
    let (mut drag, mut t) = or_return!(q.get_mut(ev.entity));
    let d = ev.delta;
    drag.drag_position += Vec2::new(d.x, -d.y);

    let target = drag.snapped_position.unwrap_or(drag.drag_position);
    t.translation = asymptotic_smoothing_with_delta_time(
        t.translation,
        target.extend(t.translation.z),
        0.4,
        time.delta_secs(),
    );
}

pub(super) fn on_draggable_drag_end<
    TDraggable: Component + InventoryPosition + Clone,
    TDropTarget: Component,
>(
    ev: On<PointerDragEnd>,
    mut piece_q: Query<
        (
            &mut Transform,
            &TDraggable,
            &mut DragPosition,
            &InventoryIndex,
        ),
        Without<IgnoreDrag>,
    >,
    mut cmd: Commands,
    layout: GameLayout,
) {
    let (mut t, draggable, mut drag, idx) = or_return!(piece_q.get_mut(ev.entity));
    if let Some(snapped_pos) = drag.snapped_position {
        let mut e_cmd = or_return!(cmd.get_entity(ev.entity));
        // set position in case it's still easing
        t.translation = snapped_pos.extend(t.translation.z);

        e_cmd.try_remove::<TDraggable>();
        e_cmd.try_insert(
            TransformScaleLensSrc::new(Vec2::ZERO)
                .duration(ms(160))
                .despawn_target_on_completion(),
        );
        e_cmd.trigger(|e| {
            DraggableDrop::<TDraggable, TDropTarget>::new(draggable.clone(), e, snapped_pos)
        });
    } else {
        cmd.try_remove_from::<DragFocus>(ev.entity);
        let start_pos = TDraggable::inventory_position(&layout, idx);
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
