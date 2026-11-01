use bevy::{ecs::component::Mutable, prelude::*};
use bevy_tweening::*;
use std::time::Duration;
use tiny_bail::or_return;

use crate::lens_src::LensSrcToLens;

pub struct TweenBuilderPlugin;
impl Plugin for TweenBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweeningPlugin)
            .add_systems(Update, tick_delay)
            .add_observer(TweenBuilder::on_insert)
            .add_observer(TweenBuilder::on_delay_removed)
            .add_observer(TweenBuilder::on_anim_completed);
    }
}

#[derive(Component)]
pub struct TweenBuilder {
    #[allow(clippy::type_complexity)]
    insert_anim_handler: Option<
        Box<
            dyn FnOnce(
                    &mut EntityCommands,
                    &EntityRef,
                    Option<EaseFunction>,
                    Duration,
                    Option<Entity>,
                ) + Send
                + Sync,
        >,
    >,
    duration: Duration,
    easing: Option<EaseFunction>,
    delay: Option<Duration>,
    target: Option<Entity>,
    despawn_target_on_completion: bool,
    // uniq_key: &'static str,
}
impl TweenBuilder {
    #[must_use]
    pub fn new<
        TComponent: Component<Mutability = Mutable>,
        TLens: Lens<TComponent> + Send + Sync + 'static,
    >(
        lens_src: impl LensSrcToLens<Component = TComponent, Lens = TLens> + Send + Sync + 'static,
        duration: Duration,
    ) -> Self {
        Self {
            easing: None,
            delay: None,
            target: None,
            despawn_target_on_completion: false,
            insert_anim_handler: Some(Box::new(move |e_cmd, e_ref, easing, duration, target| {
                let Some(component) = e_ref.get::<TComponent>() else {
                    return;
                };
                let lens = lens_src.lens(component);
                Self::insert_anim(e_cmd, lens, easing, duration, target);
            })),
            duration,
        }
    }

    #[must_use]
    pub fn from_lens<
        TComponent: Component<Mutability = Mutable>,
        TLens: Lens<TComponent> + Send + Sync + 'static,
    >(
        lens: TLens,
        duration: Duration,
    ) -> Self {
        Self {
            easing: None,
            delay: None,
            target: None,
            despawn_target_on_completion: false,
            insert_anim_handler: Some(Box::new(move |e_cmd, _e_ref, easing, duration, target| {
                Self::insert_anim(e_cmd, lens, easing, duration, target);
            })),
            duration,
        }
    }

    #[must_use]
    pub fn easing(mut self, easing: EaseFunction) -> Self {
        self.easing = Some(easing);
        self
    }

    #[must_use]
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    #[must_use]
    pub fn target(mut self, target_e: Entity) -> Self {
        self.target = Some(target_e);
        self
    }

    #[must_use]
    pub fn despawn_target_on_completion(mut self) -> Self {
        self.despawn_target_on_completion = true;
        self
    }

    fn insert_anim<TComponent: Component<Mutability = Mutable>>(
        e_cmd: &mut EntityCommands,
        lens: impl Lens<TComponent> + 'static + Sync + Send,
        easing: Option<EaseFunction>,
        duration: Duration,
        target: Option<Entity>,
    ) {
        let anim = TweenAnim::new(Tween::new(
            easing.unwrap_or(EaseFunction::QuadraticInOut),
            duration,
            lens,
        ));
        e_cmd.try_insert(anim);
        if let Some(target_e) = target {
            e_cmd.try_insert(AnimTarget::component::<TComponent>(target_e));
        }
    }

    fn on_insert(ev: On<Insert<TweenBuilder>>, mut cmd: Commands, mut e_mut_q: Query<EntityMut>) {
        let e = ev.event_target();
        let Ok(e_mut) = e_mut_q.get_mut(e) else {
            // todo: tracing
            return;
        };
        let Some(builder) = e_mut.get::<TweenBuilder>() else {
            return;
        };

        let Ok(mut e_cmd) = cmd.get_entity(e) else {
            return;
        };
        match builder.delay {
            Some(delay) => {
                e_cmd.try_insert(TweenBuilderDelay(Timer::new(delay, TimerMode::Once)));
            }
            None => {
                Self::process_handler(e, &mut cmd, &mut e_mut_q);
            }
        }
    }

    fn on_delay_removed(
        ev: On<Remove<TweenBuilderDelay>>,
        mut cmd: Commands,
        mut e_mut_q: Query<EntityMut>,
    ) {
        let e = ev.event_target();
        let Ok(e_mut) = e_mut_q.get_mut(e) else {
            // todo: tracing
            return;
        };
        let Some(delay) = e_mut.get::<TweenBuilderDelay>() else {
            return;
        };

        if delay.is_finished() {
            Self::process_handler(e, &mut cmd, &mut e_mut_q);
        }
    }

    fn process_handler(entity: Entity, cmd: &mut Commands, e_mut_q: &mut Query<EntityMut>) {
        let Ok(mut e_mut) = e_mut_q.get_mut(entity) else {
            // todo: tracing
            return;
        };
        let Some(mut builder) = e_mut.get_mut::<TweenBuilder>() else {
            return;
        };
        let handler = builder
            .insert_anim_handler
            .take()
            .expect("anim not inserted yet");
        let easing = builder.easing;
        let duration = builder.duration;
        let target = builder.target;
        let Ok(mut e_cmd) = cmd.get_entity(entity) else {
            return;
        };
        let Ok(target_e_mut) = e_mut_q.get_mut(target.unwrap_or(entity)) else {
            // todo: tracing
            return;
        };
        handler(
            &mut e_cmd,
            &target_e_mut.as_readonly(),
            easing,
            duration,
            target,
        );
    }

    fn on_anim_completed(
        ev: On<AnimCompletedEvent>,
        builder_q: Query<&TweenBuilder>,
        mut cmd: Commands,
    ) {
        let builder = or_return!(builder_q.get(ev.event_target()));
        if builder.despawn_target_on_completion {
            // despawn target upon anim completion when set
            let target = builder.target.unwrap_or(ev.anim_entity);
            or_return!(cmd.get_spawned_entity(target)).try_despawn();
        }
        if builder.target.is_some() {
            // despawn builder/animator entities that only target a component, but are not a part of it
            or_return!(cmd.get_spawned_entity(ev.anim_entity)).try_despawn();
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct TweenBuilderDelay(Timer);

fn tick_delay(
    mut cmd: Commands,
    mut builder_q: Query<(Entity, &mut TweenBuilderDelay), With<TweenBuilder>>,
    time: Res<Time>,
) {
    for (e, mut delay) in &mut builder_q {
        delay.tick(time.delta());
        if delay.just_finished() {
            let Ok(mut e_cmd) = cmd.get_entity(e) else {
                continue;
            };
            e_cmd.try_remove::<TweenBuilderDelay>();
        }
    }
}
