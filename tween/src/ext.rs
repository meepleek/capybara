use bevy::prelude::*;

use crate::tween_builder::TweenBuilder;

pub trait EntityCommandsExt {
    fn with_tween_child(&mut self, tween: TweenBuilder) -> &mut Self;
    fn with_tween_children(&mut self, tweens: impl IntoIterator<Item = TweenBuilder>) -> &mut Self;
}

impl<'a> EntityCommandsExt for EntityCommands<'a> {
    fn with_tween_child(&mut self, tween_builder: TweenBuilder) -> &mut Self {
        self.with_child(tween_builder.target(self.id()));
        self
    }

    fn with_tween_children(&mut self, tweens: impl IntoIterator<Item = TweenBuilder>) -> &mut Self {
        let e = self.id();
        self.with_children(|b| {
            for t in tweens {
                b.spawn(t.target(e));
            }
        });
        self
    }
}

pub trait CommandsExt {
    fn with_tween_child(&mut self, entity: Entity, tween: TweenBuilder) -> &mut Self;
    fn with_tween_children(
        &mut self,
        entity: Entity,
        tweens: impl IntoIterator<Item = TweenBuilder>,
    ) -> &mut Self;
}

impl<'w, 's> CommandsExt for Commands<'w, 's> {
    fn with_tween_child(&mut self, entity: Entity, tween: TweenBuilder) -> &mut Self {
        if let Ok(mut e_cmd) = self.get_entity(entity) {
            e_cmd.with_tween_child(tween);
        }
        self
    }

    fn with_tween_children(
        &mut self,
        entity: Entity,
        tweens: impl IntoIterator<Item = TweenBuilder>,
    ) -> &mut Self {
        if let Ok(mut e_cmd) = self.get_entity(entity) {
            e_cmd.with_tween_children(tweens);
        }
        self
    }
}
