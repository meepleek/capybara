use bevy::{ui::InteractionDisabled, ui_widgets::Activate};

use crate::{gameplay::ui::button, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, tick_popup_cooldown);
}

#[derive(EntityEvent)]
pub struct TextPopupClosed(Entity);

#[derive(Component, Default, Deref, DerefMut, FromTemplate)]
struct TextPopupContinuBtnCooldown(Timer);

#[derive(Component, Default, FromTemplate)]
struct TextPopup;

pub fn popup(text: impl Into<String>) -> impl Scene {
    bsn! {
        #text_popup
        TextPopup
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        Children[
            Node {
                width: vw(50),
                height: vh(60),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Column,
                column_gap: px(5),
                border: px(3),
                border_radius: px(12.),
                padding: px(20.)
            }
            BorderColor::all(Color::WHITE)
            BackgroundColor(Color::BLACK)
            Children [
                #text
                Text(text)
                --
                #continue_btn
                @button("Continue")
                TextPopupContinuBtnCooldown(Timer::from_seconds(1., TimerMode::Once))
                InteractionDisabled
                on(handle_continue)
            ]
        ]
    }
}

fn handle_continue(_ev: On<Activate>, mut cmd: Commands, popup_e: Single<Entity, With<TextPopup>>) {
    let mut e_cmd = or_return!(cmd.get_entity(*popup_e));
    e_cmd.trigger(|e| TextPopupClosed(e));
    e_cmd.despawn();
}

fn tick_popup_cooldown(
    mut btn_q: Query<(Entity, &mut TextPopupContinuBtnCooldown)>,
    time: Res<Time>,
    mut cmd: Commands,
) {
    for (e, mut timer) in &mut btn_q {
        timer.tick(time.delta());
        if timer.just_finished() {
            cmd.try_remove_from::<TextPopupContinuBtnCooldown>(e);
            cmd.try_remove_from::<InteractionDisabled>(e);
        }
    }
}
