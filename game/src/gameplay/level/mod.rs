use crate::{
    gameplay::{
        cauldron::{Cauldron, cauldron_scene},
        customer::{CustomerKind, spawn::customer_scene},
        grid::spawn::grid_scene,
        ingredient::CommonIngredient,
        layout::GameLayout,
        level::{
            regular::{LevelData, LevelStats},
            tutorial::{TutorialLevelData, TutorialState},
        },
        piece::bag::PieceBag,
    },
    prelude::*,
    screens::GameplayLevel,
};

pub mod regular;
pub mod spawn;
pub mod tutorial;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((tutorial::plugin, spawn::plugin, regular::plugin));
    app.insert_resource(LevelIndex(2));
    // app.insert_resource(LevelIndex(0));
    app.add_observer(level_finished);
    app.add_systems(OnEnter(Screen::Gameplay), setup_level);
}

#[derive(Resource)]
pub struct LevelIndex(pub u8);

#[derive(Component, Debug)]
pub struct LevelState {
    pub spawned_order_count: u8,
}

#[derive(Event)]
pub struct LevelFinished;

#[derive(Debug, Clone)]
pub enum LevelKind {
    Tutorial(TutorialLevelData),
    Regular(LevelData),
}
impl LevelKind {
    pub fn levels() -> Vec<LevelKind> {
        use LevelKind::*;

        vec![
            Tutorial(TutorialLevelData::intro_to_basics()),
            Regular(LevelData::basics()),
            Tutorial(TutorialLevelData::ingredient_abilities()),
        ]
    }

    pub fn level(idx: u8) -> LevelKind {
        Self::levels()
            .iter()
            .nth(idx as usize)
            .cloned()
            .expect("invalid level idx")
    }

    pub fn level_count() -> u8 {
        Self::levels().len() as u8
    }
}

impl CommonIngredient {
    pub fn min_lvl(&self) -> u8 {
        use CommonIngredient::*;

        match self {
            Apple | Orange | Water => 1,
            Ginger => 3,
            Honey => 6,
            Coffee => 9,
            Air => 12,
        }
    }
}

impl Cauldron {
    pub fn count(level_idx: u8) -> u8 {
        match level_idx {
            0..8 => 1,
            _ => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Tool {
    IngredientFuser,
}
impl Tool {
    pub fn min_lvl(&self) -> u8 {
        use Tool::*;

        match self {
            IngredientFuser => 5,
        }
    }
}

fn setup_level(
    mut cmd: Commands,
    lvl_idx: Res<LevelIndex>,
    layout: GameLayout,
    mut next_level_type: ResMut<NextState<GameplayLevel>>,
) {
    let mut rng = rng();
    let lvl_idx = lvl_idx.0;
    let level = LevelKind::level(lvl_idx);
    match level {
        LevelKind::Tutorial(tutorial) => {
            next_level_type.set(GameplayLevel::Tutorial);
            cmd.spawn_scene(bsn! {
                #tutorial_state
                TutorialState {
                    steps: {tutorial.steps}
                }
            });
            // grid
            cmd.spawn_scene(grid_scene(
                &layout,
                Some(tutorial.initial_board.grid),
                lvl_idx,
            ));
            // customer
            let customer_idx = 0;
            let pos = layout.customer_position(customer_idx);
            cmd.spawn_scene(customer_scene(
                customer_idx,
                pos.extend(1.),
                CustomerKind::default(),
            ));
            // cauldron
            if tutorial.initial_board.show_cauldron {
                let cauldron_idx = 0;
                cmd.spawn_scene(cauldron_scene(
                    layout.cauldron_position(cauldron_idx),
                    cauldron_idx,
                ));
            }
        }
        LevelKind::Regular(level_data) => {
            next_level_type.set(GameplayLevel::Regular);
            cmd.spawn((
                PieceBag::new(
                    level_data.piece_bag.clone(),
                    level_data.piece_bag_random_count,
                ),
                LevelStats::default(),
                level_data.clone(),
            ));
            // grid
            cmd.spawn_scene(grid_scene(&layout, None, lvl_idx));
            // customers
            let mut customers: Vec<_> = level_data.customers.iter().cloned().collect();
            customers.shuffle(&mut rng);
            for (i, kind) in customers.into_iter().enumerate() {
                let pos = layout.customer_position(i as u8);
                cmd.spawn_scene(customer_scene(i as u8, pos.extend(1.), kind));
            }
            // cauldron(s)
            for i in 0..Cauldron::count(lvl_idx) {
                cmd.spawn_scene(cauldron_scene(layout.cauldron_position(i), i));
            }
        }
    }
}

fn level_finished(
    _ev: On<LevelFinished>,
    mut next: ResMut<NextState<Screen>>,
    mut lvl_idx: ResMut<LevelIndex>,
) {
    if lvl_idx.0 + 1 < LevelKind::level_count() {
        lvl_idx.0 += 1;
        next.set(Screen::Gameplay);
    } else {
        todo!("u win screen!")
    }
}
