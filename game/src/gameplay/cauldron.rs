use mplk_ecs::relationship_1_to_n;

use crate::{
    gameplay::{
        drag::DraggableDrop,
        grid::IngredientGrid,
        ingredient::Ingredient,
        layout::{GameLayout, LayoutElementPosition},
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
    app.add_observer(on_ingredient_drop);
}

// #[derive(Component, Debug, Clone, FromTemplate)]
// pub struct Cauldron;
impl LayoutElementPosition for Cauldron {
    fn position(layout: &GameLayout, _grid: &IngredientGrid) -> Vec2 {
        layout.shaker_position()
    }
}

relationship_1_to_n!(CauldronIngredient, Cauldron);

fn setup(mut cmd: Commands, layout: GameLayout) {
    cmd.spawn_scene(shaker_scene(layout.shaker_position()));
}

fn shaker_scene(position: Vec2) -> impl Scene {
    bsn! {
        #shaker
        Cauldron
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

fn on_ingredient_drop(
    ev: On<DraggableDrop<Ingredient, Cauldron>>,
    layout: GameLayout,
    mut cmd: Commands,
    cauldron_q: Query<&Cauldron>,
) {
    let idx = cauldron_q.get(ev.target_entity).map_or(0, |c| c.0.len()) as u8;
    let position = layout.cauldron_ingredient_position(idx);
    let icon = ev.draggable.icon_scene(position.extend(0.));
    // let tween = TransformScaleLensSrc::absolute(Vec2::ZERO, Vec2::ONE)
    //     .duration(ms(160))
    //     .despawn_target_on_completion();
    cmd.spawn_scene(bsn! {
        #cauldron_ingredient
        ChildOf({ev.target_entity})
        CauldronIngredient({ev.target_entity})
        @{icon}
    });
}
