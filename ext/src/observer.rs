use bevy::ecs::relationship::Relationship;
use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;

pub trait EntityCommandsObserverExt {
    fn observe_with_relationship<R: Relationship, E: Event, M>(
        &mut self,
        observer: impl IntoObserverSystem<E, M>,
    ) -> &mut Self;
}
impl EntityCommandsObserverExt for EntityCommands<'_> {
    fn observe_with_relationship<R: Relationship, E: Event, M>(
        &mut self,
        observer: impl IntoObserverSystem<E, M>,
    ) -> &mut Self {
        let e = self.id();
        let observer = Observer::new(observer).with_entity(e);
        self.commands_mut().spawn(observer).add_one_related::<R>(e);
        self
    }
}
