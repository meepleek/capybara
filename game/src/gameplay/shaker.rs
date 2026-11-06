use crate::{
    gameplay::{
        grid::IngredientGrid,
        layout::{GameLayout, LayoutElementPosition},
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

#[derive(Component, Debug, Clone, FromTemplate)]
pub struct Shaker;
impl LayoutElementPosition for Shaker {
    fn position(layout: &GameLayout, _grid: &IngredientGrid) -> Vec2 {
        layout.shaker_position()
    }
}

fn setup(mut cmd: Commands, layout: GameLayout) {
    cmd.spawn_scene(shaker_scene(layout.shaker_position()));
}

fn shaker_scene(position: Vec2) -> impl Scene {
    bsn! {
        #shaker
        Shaker
        Transform { translation: {position.extend(1.)} }
        from_collection(move |sprites: &SpriteAssets| Sprite {
            image: sprites.bar_sheet.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: sprites.bar_layout.clone(),
                index: 0
            }),
            custom_size: Some(Vec2::splat(256.)),
            ..default()
        })
        Pickable
        DespawnOnExit<Screen>(Screen::Gameplay)
    }
}
