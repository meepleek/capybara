use crate::{
    gameplay::{ingredient::CommonIngredient, level::Tool, piece::template::PieceTemplate},
    prelude::*,
};

pub(super) fn plugin(_app: &mut App) {}

pub struct TutorialLevelData {
    board: TutorialBoardState,
    orders: Vec<Vec<CommonIngredient>>,
    steps: Vec<TutorialStep>,
}

pub enum TutorialStep {
    Text(Vec<String>),
    AddIngredient(CommonIngredient),
    WaitForIngredient(CommonIngredient),
    DequeueOrder,
    ShowTool(Tool),
}

pub struct TutorialBoardState {
    occupied_grid: String,
    // unlike regular lvls allows dealing out less pieces
    pieces: Vec<Vec<PieceTemplate>>,
    piece_bag: Vec<PieceTemplate>,
    ingredients: Vec<CommonIngredient>,
    tools: Vec<Tool>,
}
