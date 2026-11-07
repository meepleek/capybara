use crate::{
    gameplay::{
        grid::spawn::grid_scene,
        ingredient::CommonIngredient,
        layout::GameLayout,
        level::{regular::LevelData, tutorial::TutorialLevelData},
    },
    prelude::*,
};

pub mod regular;
pub mod spawn;
pub mod tutorial;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((tutorial::plugin, spawn::plugin, regular::plugin));
    app.insert_resource(LevelIndex(0));
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

        vec![Tutorial(TutorialLevelData::intro_to_basics())]
    }

    pub fn level(idx: u8) -> LevelKind {
        Self::levels()
            .iter()
            .nth(idx as usize)
            .cloned()
            .expect("invalid level idx")
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

fn setup_level(mut cmd: Commands, lvl_idx: Res<LevelIndex>, layout: GameLayout) {
    let level = LevelKind::level(lvl_idx.0);
    match level {
        LevelKind::Tutorial(tutorial) => {
            cmd.spawn_scene(grid_scene(
                &layout,
                Some(tutorial.initial_board.grid),
                lvl_idx.0,
            ));
        }
        LevelKind::Regular(_level_data) => {}
    }
}

fn level_finished(_ev: On<LevelFinished>) {}
