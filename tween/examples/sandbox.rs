use bevy::{color::palettes::tailwind, prelude::*};

use mplk_tween::prelude::*;
use mplk_utils::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, TweenBuilderPlugin))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut cmd: Commands) {
    cmd.spawn(Camera2d);
    let start = Color::from(tailwind::AMBER_300);
    cmd.spawn((
        Sprite::from_color(start, Vec2::splat(100.)),
        TweenBuilder::from_lens(
            lens::SpriteColorLens {
                start,
                end: tailwind::CYAN_700.into(),
            },
            ms(400),
        )
        .delay(ms(150)),
    ));

    cmd.spawn((
        Sprite::from_color(tailwind::GRAY_600, Vec2::splat(100.)),
        Transform::from_translation(Vec3::NEG_X * 200.),
        SpriteColorLensSrc::absolute(tailwind::GREEN_400, tailwind::AMBER_400)
            .duration(ms(800))
            .delay(ms(1050)),
    ));

    cmd.spawn((
        Sprite::from_color(tailwind::RED_600, Vec2::splat(100.)),
        Transform::from_translation(Vec3::X * 200.)
            .with_rotation(Quat::from_rotation_z(45f32.to_radians())),
    ))
    .with_tween_children([
        TransformRotationByDegrees(920.)
            .duration(ms(2600))
            .delay(ms(500))
            .despawn_target_on_completion(),
        TransformPositionMoveBy(Vec2::Y * 200.)
            .duration(ms(1500))
            .delay(ms(500))
            .despawn_target_on_completion(),
    ]);
}
