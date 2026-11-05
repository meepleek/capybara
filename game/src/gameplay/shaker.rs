use bevy::window::WindowResized;

use crate::{gameplay::layout::GameLayout, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
    app.add_systems(
        Update,
        move_shaker_on_resize.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component, Debug, Clone, FromTemplate)]
pub struct Shaker;

fn setup(mut cmd: Commands, layout: GameLayout) {
    cmd.spawn_scene(shaker_scene(layout.shaker_position()));
}

fn shaker_scene(position: Vec2) -> impl Scene {
    bsn! {
        #shaker
        Shaker
        Transform { translation: {position.extend(0.1)} }
        from_collection(move |sprites: &SpriteAssets| Sprite {
            image: sprites.bar_sheet.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: sprites.bar_layout.clone(),
                index: 0
            }),
            custom_size: Some(Vec2::splat(256.)),
            ..default()
        })
        DespawnOnExit<Screen>(Screen::Gameplay)
        Pickable
    }
}

fn move_shaker_on_resize(
    mut resize_mr: MessageReader<WindowResized>,
    mut transform_q: Query<&mut Transform, With<Shaker>>,
    layout: GameLayout,
) {
    if resize_mr.clear_any() {
        for mut t in &mut transform_q {
            t.translation = layout.shaker_position().extend(t.translation.z);
        }
    }
}
