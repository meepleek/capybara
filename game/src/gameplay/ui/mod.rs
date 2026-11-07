use crate::prelude::*;
use bevy::ui_widgets::Button;

pub mod text_popup;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(text_popup::plugin);
}

pub fn ui_root() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: px(5),
        }
    }
}

pub fn button(label: &str) -> impl Scene {
    bsn! {
        Button
        Node {
            border: px(5),
            border_radius: px(12),
            padding: px(12),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BorderColor::from(Color::BLACK)
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
        Children [
            Text(label)
            TextFont {
                font_size: px(33.0),
            }
            TextColor(Color::srgb(0.9, 0.9, 0.9))
            TextShadow
        ]
    }
}
