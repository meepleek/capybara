use mplk_ecs::relationship_1_to_n;

use crate::{
    gameplay::{
        drag::DraggableDrop,
        ingredient::Ingredient,
        inventory::InventoryIndex,
        layout::{GameLayout, InventoryPosition},
    },
    prelude::*,
    screens::run_conditions::in_game,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, tick_cauldron_timer.run_if(in_game));
    app.add_observer(on_ingredient_drop)
        .add_observer(readd_cauldron);
}

impl InventoryPosition for Cauldron {
    fn inventory_position(layout: &GameLayout, idx: &gameplay::inventory::InventoryIndex) -> Vec2 {
        layout.cauldron_position(idx.0)
    }
}

relationship_1_to_n!(IngredientOfCauldron, Cauldron);

#[derive(Component, Deref, DerefMut, Clone, FromTemplate)]
pub struct CauldronTimer(Timer);
impl Default for CauldronTimer {
    fn default() -> Self {
        Self(Timer::new(sec(5.), TimerMode::Once))
    }
}

#[derive(Component, Deref, DerefMut, Clone, FromTemplate)]
pub struct CauldronIngredient(pub Ingredient);

#[derive(Event)]
pub struct DrinkReady {
    pub ingredients: Vec<Ingredient>,
}

#[derive(Event, Debug)]
pub struct OrderlessDrink {
    pub ingredients: Vec<Ingredient>,
}

pub fn cauldron_scene(position: Vec2, idx: u8) -> impl Scene {
    bsn! {
        #cauldron
        Cauldron
        InventoryIndex(idx)
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
    mut timer_q: Query<&mut CauldronTimer>,
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
        IngredientOfCauldron({ev.target_entity})
        CauldronIngredient({ev.draggable})
        @{icon}
    });

    match timer_q.get_mut(ev.target_entity) {
        Ok(mut timer) => {
            // adding an ingredient resets the timer a bit
            let elapsed = timer.elapsed().saturating_sub(sec(1.));
            timer.set_elapsed(elapsed);
        }
        Err(_) => {
            cmd.try_insert_to(ev.target_entity, CauldronTimer::default());
        }
    };
    if idx == 0 {}
}

fn tick_cauldron_timer(
    mut timer_q: Query<(Entity, &Cauldron, &mut CauldronTimer)>,
    ingredient_q: Query<&CauldronIngredient>,
    time: Res<Time>,
    mut cmd: Commands,
) {
    for (e, cauldron, mut timer) in &mut timer_q {
        timer.tick(time.delta());
        if timer.just_finished() {
            let mut ingredients = Vec::new();
            for ingredient_e in &cauldron.0 {
                let ingredient = or_continue!(ingredient_q.get(*ingredient_e)).0;
                ingredients.push(ingredient);
                let mut e_cmd = or_continue!(cmd.get_entity(*ingredient_e));
                e_cmd.try_remove::<CauldronIngredient>();
                e_cmd.try_remove::<IngredientOfCauldron>();
                e_cmd.try_insert(
                    TransformScaleLensSrc::new(Vec2::ZERO)
                        .duration(ms(160))
                        .despawn_target_on_completion(),
                );
            }
            tracing::info!(?ingredients, "drink done");
            cmd.trigger(DrinkReady { ingredients });
            cmd.try_remove_from::<CauldronTimer>(e);
        }
    }
}

// todo: handle this in a more generic way
fn readd_cauldron(ev: On<Remove<Cauldron>>, mut cmd: Commands) {
    cmd.try_insert_to(ev.entity, Cauldron::default());
}
