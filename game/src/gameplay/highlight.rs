use bevy::ecs::system::SystemParam;
use mplk_ecs::relationship_1_to_1;

use crate::{
    gameplay::drag::{DragFocus, DraggableSnapChange, ValidDragTarget},
    prelude::*,
};

pub struct DragHighlightPlugin<
    TDraggable: Component,
    TDragTarget: Component + ValidDragTarget<TDraggable>,
>(PhantomData<TDraggable>, PhantomData<TDragTarget>);
// Default implemented manually to avoid extra trait bounds from derive
impl<TDraggable: Component, TDragTarget: Component + ValidDragTarget<TDraggable>> Default
    for DragHighlightPlugin<TDraggable, TDragTarget>
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}
impl<TDraggable: Component, TDragTarget: Component + ValidDragTarget<TDraggable>> Plugin
    for DragHighlightPlugin<TDraggable, TDragTarget>
{
    fn build(&self, app: &mut App) {
        app.add_observer(Self::highlight_target_on_snap);
        app.add_systems(Update, Self::highlight_target_on_draggable_focus_change);
    }
}
impl<TDraggable: Component, TDragTarget: Component + ValidDragTarget<TDraggable>>
    DragHighlightPlugin<TDraggable, TDragTarget>
{
    fn highlight_target_on_snap(
        ev: On<DraggableSnapChange<TDraggable, TDragTarget>>,
        target_q: Query<(), With<TDragTarget>>,
        palette: Res<HighlightPalette>,
        mut highlights: Highlights,
    ) {
        or_return!(target_q.get(ev.target_entity));
        let snapped = ev.position.is_some();
        highlights.tween(
            ev.draggable_entity,
            if snapped {
                palette.draggable_snap
            } else {
                Color::NONE
            },
        );
        highlights.tween(
            ev.target_entity,
            if snapped {
                palette.target_draggable_snap
            } else {
                Color::NONE
            },
        );
    }

    fn highlight_target_on_draggable_focus_change(
        draggable_changed_q: Query<(Entity, &TDraggable, &DragFocus), Changed<DragFocus>>,
        mut draggable_removed: RemovedComponents<TDraggable>,
        target_q: Query<Entity, With<TDragTarget>>,
        palette: Res<HighlightPalette>,
        mut highlights: Highlights,
    ) {
        let mut to_update = None;
        if let Some((draggable_e, _, focus)) = draggable_changed_q
            .iter()
            .filter(|(_, draggable, _)| TDragTarget::valid(draggable))
            .next()
        {
            let (draggable_col, target_col) = match focus {
                DragFocus::Hover => (palette.draggable_hover, palette.target_draggable_hover),
                DragFocus::Active => (palette.draggable_active, palette.target_draggable_active),
            };
            to_update = Some((draggable_e, draggable_col, target_col));
        } else if let Some(e) = draggable_removed.read().next() {
            to_update = Some((e, Color::NONE, Color::NONE));
        }
        let Some((draggable_e, draggable_col, target_col)) = to_update else {
            return;
        };
        highlights.tween(draggable_e, draggable_col);
        for target_e in &target_q {
            highlights.tween(target_e, target_col);
        }
    }
}

relationship_1_to_1!(HighlightOf, HighlightRoot);

#[derive(Resource, Debug)]
pub struct HighlightPalette {
    pub draggable_hover: Color,
    pub draggable_active: Color,
    pub draggable_snap: Color,
    pub target_draggable_hover: Color,
    pub target_draggable_active: Color,
    pub target_draggable_snap: Color,
}

#[derive(SystemParam)]
struct Highlights<'w, 's> {
    cmd: Commands<'w, 's>,
    highlight_root_q: Query<'w, 's, &'static HighlightRoot>,
}
impl Highlights<'_, '_> {
    const TWEEN_DURATION: Duration = ms(300);

    fn tween(&mut self, entity: Entity, color: Color) {
        let e = self.highlight_root_q.get(entity).map_or(entity, |h| h.0);
        self.cmd.try_insert_to(
            e,
            OutlineColorLensSrc::new(color).duration(Self::TWEEN_DURATION),
        );
    }
}
