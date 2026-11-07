use mplk_grid::{bit_grid::BitGrid, ext::ParsedGrid};

use crate::{
    gameplay::{
        grid::INGREDIENT_TILE_SIZE,
        ingredient::{CommonIngredient, Ingredient},
        level::Tool,
        piece::template::PieceTemplate,
    },
    prelude::*,
};

pub(super) fn plugin(_app: &mut App) {}

#[derive(Debug, Clone)]
pub struct TutorialLevelData {
    pub initial_board: TutorialBoardState,
    pub steps: Vec<TutorialStep>,
}
impl TutorialLevelData {
    pub fn intro_to_basics() -> TutorialLevelData {
        use CommonIngredient::*;
        use Ingredient::*;
        use PieceTemplate::*;
        use TutorialStep::*;

        let grid = r"
.####.###
#.##..###
.####.###
#.#######
.########
#.#######
.########
#.#.#.#.#
.#.#.#.#.
";
        TutorialLevelData {
            initial_board: TutorialBoardState {
                grid: Self::parse_grid(grid),
                tools: vec![],
            },
            steps: vec![
                TutorialStep::text(
                    "Welcome, dear capybarista.\nLet's take a stab at making your first drink.",
                ),
                Order(vec![Common(Orange)]),
                TutorialStep::text(
                    "Oh look, our first order has come in. Let's get cracking then.",
                ),
                TutorialStep::text("On your left is the ingredient board."),
                AddPiece {
                    template: T,
                    cw_rotations: 0,
                },
                TutorialStep::text(
                    "Drag pieces to fill in rows, columns and rows to synthentize whatever is shown in or next to those areas.",
                ),
                WaitForIngredient(Common(Orange)),
                TutorialStep::text(
                    "Now you can grab those ingredients and drag them to the cauldron to fulfill the order.",
                ),
                WaitForFinishedOrder,
                TutorialStep::text("Nicely done!\nNow try one more order."),
                Order(vec![Common(Apple), Common(Orange)]),
                TutorialStep::text("Nicely done!\nNow try one more order."),
                WaitForFinishedOrder,
                TutorialStep::text("Good work!\nSee you soon for your first full shift."),
            ],
        }
    }

    pub fn intro_to_ingredient_abilities() -> TutorialLevelData {
        todo!()
    }

    pub fn intro_to_fusing() -> TutorialLevelData {
        todo!()
    }

    fn parse_grid(grid: &str) -> BitGrid {
        let parsed: ParsedGrid = grid.parse().expect("invalid tutorial board");
        let mut bitgrid = BitGrid::new(parsed.grid_size, INGREDIENT_TILE_SIZE);
        for t in parsed.occupied_tiles {
            bitgrid.place_tile(t).expect("invalid tutorial grid tile");
        }
        bitgrid
    }
}

#[derive(Debug, Clone)]
pub enum TutorialStep {
    Text(String),
    AddPiece {
        template: PieceTemplate,
        cw_rotations: u8,
    },
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

#[derive(Debug, Clone)]
pub struct TutorialBoardState {
    // todo: make this a bitgrid?
    pub grid: BitGrid,
    pub tools: Vec<Tool>,
}
