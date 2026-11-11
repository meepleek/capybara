use mplk_ecs::observer::{ensure_single_at_most, map_entity_event};
use mplk_utils::math::asymptotic_smoothing_with_delta_time;

use crate::{
    gameplay::{
        inventory::InventoryIndex,
        layout::{GameLayout, InventoryPosition},
        ui::text_popup::TextPopupOpened,
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(ensure_single_at_most::<DragFocus>);
    app.add_systems(Update, (raise_z_on_drag_active, smooth_drag))
        .add_observer(restore_z_on_drag_rmv)
        .add_observer(cancel_drag::<TextPopupOpened>)
        .add_observer(map_entity_event::<PointerDragEnd, _>(|e, _| DragEnd(e)));
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

#[derive(Debug, Clone)]
pub struct SnapData {
    pub position: Vec2,
    pub entity: Entity,
}

#[derive(Debug, Component, Clone, Default)]
#[require(Transform)]
pub struct DragPosition {
    pub drag_position: Vec2,
    pub snapped_data: Option<SnapData>,
}
impl DragPosition {
    pub fn new(drag_position: Vec2) -> Self {
        Self {
            drag_position,
            snapped_data: None,
        }
    }
}

#[derive(EntityEvent, Debug)]
pub struct DragEnd(pub Entity);

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DropTarget<T>(PhantomData<T>);

#[derive(EntityEvent, Debug, PartialEq)]
pub struct DraggableDrop<TDraggable: Component + InventoryPosition + Clone, TDropTarget: Component>
{
    pub position: Vec2,
    #[event_target]
    pub target_entity: Entity,
    pub draggable: TDraggable,
    _drop_phantom: PhantomData<TDropTarget>,
}
impl<TDraggable: Component + InventoryPosition + Clone, TDropTarget: Component>
    DraggableDrop<TDraggable, TDropTarget>
{
    pub fn new(draggable: TDraggable, target_entity: Entity, position: Vec2) -> Self {
        Self {
            position,
            target_entity,
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

pub(super) fn on_draggable_drag_end_snapped<
    TDraggable: Component + InventoryPosition + Clone + Debug,
    TDropTarget: Component,
>(
    ev: On<PointerDragEnd>,
    mut piece_q: Query<(&mut Transform, &TDraggable, &DragPosition), Without<IgnoreDrag>>,
    target_q: Query<(), With<TDropTarget>>,
    mut cmd: Commands,
) {
    let (mut t, draggable, drag) = or_return!(piece_q.get_mut(ev.entity));
    if let Some(snap_data) = drag.snapped_data.as_ref() {
        if !target_q.contains(snap_data.entity) {
            return;
        }
        let mut dragged_e_cmd = or_return!(cmd.get_entity(ev.entity));
        // set position in case it's still easing
        t.translation = snap_data.position.extend(t.translation.z);

        dragged_e_cmd.try_remove::<TDraggable>();
        dragged_e_cmd.try_insert(
            TransformScaleLensSrc::new(Vec2::ZERO)
                .duration(ms(160))
                .despawn_target_on_completion(),
        );
        let mut target_e_cmd = or_return!(cmd.get_entity(snap_data.entity));
        target_e_cmd.trigger(|e| {
            DraggableDrop::<TDraggable, TDropTarget>::new(draggable.clone(), e, snap_data.position)
        });
    }
}

pub(super) fn on_draggable_drag_end_restore_position<
    TDraggable: Component + InventoryPosition + Clone + Debug,
>(
    ev: On<DragEnd>,
    mut piece_q: Query<
        (&mut DragPosition, &InventoryIndex),
        (Without<IgnoreDrag>, With<TDraggable>),
    >,
    mut cmd: Commands,
    layout: GameLayout,
) {
    let (mut drag, idx) = or_return!(piece_q.get_mut(ev.0));
    if let None = drag.snapped_data.as_ref() {
        cmd.try_remove_from::<DragFocus>(ev.0);
        let start_pos = TDraggable::inventory_position(&layout, idx);
        drag.drag_position = start_pos;
        let tween_duration = ms(330);
        cmd.try_insert_to(
            ev.0,
            (
                TransformPositionLensSrc::new(start_pos)
                    .duration(tween_duration)
                    .easing(EaseFunction::BackOut),
                IgnoreDrag,
            ),
        );
        cmd.delayed()
            .duration(tween_duration)
            .entity(ev.0)
            .try_remove::<IgnoreDrag>();
    }
}

fn smooth_drag(
    mut draggable_q: Query<(&DragPosition, &mut Transform, &DragFocus), Without<IgnoreDrag>>,
    time: Res<Time>,
) {
    for (drag, mut t, focus) in &mut draggable_q {
        if *focus != DragFocus::Active {
            continue;
        }
        let target = drag
            .snapped_data
            .as_ref()
            .map(|s| s.position)
            .unwrap_or(drag.drag_position);
        t.translation = asymptotic_smoothing_with_delta_time(
            t.translation,
            target.extend(t.translation.z),
            0.4,
            time.delta_secs(),
        );
    }
}

fn cancel_drag<TEv: EntityEvent>(
    _ev: On<TEv>,
    focus_q: Query<Entity, With<DragFocus>>,
    mut cmd: Commands,
) {
    for e in &focus_q {
        cmd.try_trigger(e, |e| DragEnd(e));
    }
}
