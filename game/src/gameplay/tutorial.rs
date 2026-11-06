use crate::{
    gameplay::{
        ingredient::{CommonIngredient, Ingredient},
        level::Tool,
        piece::template::PieceTemplate,
    },
    prelude::*,
};

pub(super) fn plugin(_app: &mut App) {}

pub struct TutorialLevelData {
    board: TutorialBoardState,
    steps: Vec<TutorialStep>,
}
impl TutorialLevelData {
    pub fn intro_to_basics() -> TutorialLevelData {
        use CommonIngredient::*;
        use Ingredient::*;
        use PieceTemplate::*;
        use TutorialStep::*;

        let grid = r"
xxxxxxxxx
xxxxxxxxx
xxxxxxxxx
xxxxxxxxx
xxxxxxxxx
xxxxxxxxx
xxxxxxxxx
xxxxxxxxx
xxxxxxxxx
";

        TutorialLevelData {
            board: TutorialBoardState {
                occupied_grid: grid.to_string(),
                pieces: vec![vec![T]],
                piece_bag: vec![Line3],
                ingredients: vec![],
                tools: vec![],
            },
            steps: vec![
                TutorialStep::text(
                    "Welcome, dear capybarista.\nLet's take a stab at your first drink.",
                ),
                Order(vec![Common(Apple), Common(Orange)]),
                TutorialStep::text(
                    "Oh look, our first order has appeared. Let's get cracking then.",
                ),
                TutorialStep::text(
                    "On your left is the ingredient board.\nDrag pieces to fill in rows, columns and rows to synthentize whatever is shown in or next to those areas.",
                ),
                WaitForIngredient(Common(Apple)),
                TutorialStep::text(
                    "Now you can grab those ingredients and drag them to the cauldron to fulfill the order.",
                ),
                WaitForFinishedOrder,
                TutorialStep::text("Nicely done.\nSee you soon for your first proper shift."),
            ],
        }
    }

    pub fn intro_to_ingredient_abilities() -> TutorialLevelData {
        todo!()
    }

    pub fn intro_to_fusing() -> TutorialLevelData {
        todo!()
    }
}

pub enum TutorialStep {
    Text(String),
    AddIngredient(CommonIngredient),
    WaitForIngredient(Ingredient),
    Order(Vec<Ingredient>),
    WaitForFinishedOrder,
    ShowTool(Tool),
}
impl TutorialStep {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(text.into())
    }
}

pub struct TutorialBoardState {
    // todo: make this a bitgrid?
    occupied_grid: String,
    // unlike regular lvls allows dealing out less pieces
    pieces: Vec<Vec<PieceTemplate>>,
    piece_bag: Vec<PieceTemplate>,
    ingredients: Vec<CommonIngredient>,
    tools: Vec<Tool>,
}
