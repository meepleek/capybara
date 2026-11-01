use bevy::{ecs::component::Mutable, prelude::*};
use std::marker::PhantomData;

#[derive(Default)]
pub struct TrackPositionPlugin<T: Component<Mutability = Mutable> + TrackPosition>(PhantomData<T>);
impl<T: Component<Mutability = Mutable> + TrackPosition> Plugin for TrackPositionPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            track_position::<T>.after(TransformSystems::Propagate),
        );
    }
}

pub trait TrackPosition {
    fn position(&self) -> Vec2;
    fn set_position(&mut self, position: Vec2);
}

fn track_position<T: Component<Mutability = Mutable> + TrackPosition>(
    mut track_q: Query<(&GlobalTransform, &mut T), Changed<GlobalTransform>>,
) {
    for (t, mut trackable) in &mut track_q {
        trackable.set_position(t.translation().truncate());
    }
}

#[macro_export]
macro_rules! impl_track_position {
    ($type: ty, $position_fld: ident) => {
        impl $crate::transform::TrackPosition for $type {
            fn position(&self) -> Vec2 {
                self.$position_fld
            }

            fn set_position(&mut self, position: Vec2) {
                self.$position_fld = position;
            }
        }
    };
}
