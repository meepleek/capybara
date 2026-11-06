use crate::{
    gameplay::{
        customer::{CustomerKind, order::OrderMissed},
        ingredient::CommonIngredient,
        piece::template::PieceTemplate,
        tutorial::TutorialLevelData,
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(processed_missed_order);
}

pub enum LevelKind {
    Tutorial(TutorialLevelData),
    Regular(LevelData),
}
impl LevelKind {
    pub fn levels() -> Vec<LevelKind> {
        use LevelKind::*;

        vec![Tutorial(TutorialLevelData::intro_to_basics())]
    }
}

#[expect(dead_code)]
pub struct LevelData {
    piece_bag: Vec<PieceTemplate>,
    piece_bag_random_count: u8,
    concurrent_order_max: u8,
    order_count: u8,
    customers: [CustomerKind; 3],
    cauldron_count: u8,
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

fn processed_missed_order(_ev: On<OrderMissed>, mut next: ResMut<NextState<Screen>>) {
    next.set(Screen::GameOver);
}
