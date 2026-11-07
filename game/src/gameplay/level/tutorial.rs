use std::collections::VecDeque;

use bevy::ui_widgets::Activate;
use mplk_ecs::observer::ensure_single_at_most;
use mplk_grid::{bit_grid::BitGrid, ext::ParsedGrid};

use crate::{
    gameplay::{
        customer::order::NewOrder,
        grid::INGREDIENT_TILE_SIZE,
        ingredient::{self, CommonIngredient, Ingredient},
        level::{LevelFinished, Tool},
        piece::template::PieceTemplate,
        ui::{
            text_popup::{TextPopupClosed, popup},
            ui_root,
        },
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(ensure_single_at_most::<TutorialState>)
        .add_observer(start_tutorial)
        .add_observer(next_step)
        .add_observer(process_step);
}

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
                TutorialStep::piece(T),
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

#[derive(Component, Debug, Clone)]
pub enum TutorialStep {
    Text(String),
    AddPiece {
        template: PieceTemplate,
        cw_rotations: u8,
    },
    AddIngredient(Ingredient),
    WaitForIngredient(Ingredient),
    Order(Vec<Ingredient>),
    WaitForFinishedOrder,
    ShowTool(Tool),
}
impl TutorialStep {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(text.into())
    }

    pub fn piece(piece: PieceTemplate) -> Self {
        Self::rotated_piece(piece, 0)
    }

    pub fn rotated_piece(piece: PieceTemplate, cw_rotations: u8) -> Self {
        Self::AddPiece {
            template: piece,
            cw_rotations,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TutorialBoardState {
    pub grid: BitGrid,
    pub tools: Vec<Tool>,
}

#[derive(Component, Debug, Clone, FromTemplate)]
pub struct TutorialState {
    pub steps: VecDeque<TutorialStep>,
}

#[derive(Event)]
pub struct NextTutorialStep;

fn start_tutorial(_ev: On<Insert<TutorialState>>, mut cmd: Commands) {
    cmd.trigger(NextTutorialStep);
}

fn next_step(
    _ev: On<NextTutorialStep>,
    mut tut_state: Single<&mut TutorialState>,
    mut cmd: Commands,
) {
    cmd.despawn_all::<With<TutorialStep>>();
    match tut_state.steps.pop_front() {
        Some(step) => {
            cmd.spawn_scene(bsn! {
                #tutorial_step
                step
            });
        }
        None => {
            cmd.trigger(LevelFinished);
        }
    };
}

fn process_step(ev: On<Insert<TutorialStep>>, step_q: Query<&TutorialStep>, mut cmd: Commands) {
    let step = or_return!(step_q.get(ev.entity));
    let next_step_delay = match step {
        TutorialStep::Text(text) => {
            let btn_handler = bsn! {
                    on(|_ev: On<Activate>| { tracing::warn!("continue pls"); })
            };
            cmd.spawn_scene(bsn! {
                #tutorial_text
                @{popup(text)}
                DespawnOnExit<Screen>(Screen::Gameplay)
            })
            .observe(handle_text_popup_closed);
            None
        }
        TutorialStep::AddPiece {
            template,
            cw_rotations,
        } => todo!(),
        TutorialStep::AddIngredient(ingredient) => {
            cmd.trigger(ingredient::AddIngredient(*ingredient));
            Some(sec(1.))
        }
        TutorialStep::WaitForIngredient(ingredient) => todo!(),
        TutorialStep::Order(ingredients) => {
            cmd.trigger(NewOrder {
                ingredients: ingredients.clone(),
                duration: todo!(),
                customer_e: todo!(),
            });
        }
        TutorialStep::WaitForFinishedOrder => todo!(),
        TutorialStep::ShowTool(tool) => todo!(),
    };
    if let Some(delay) = next_step_delay {
        cmd.delayed().duration(delay).trigger(NextTutorialStep);
    }
}

fn handle_text_popup_closed(_ev: On<TextPopupClosed>, mut cmd: Commands) {
    cmd.delayed().secs(0.5).trigger(NextTutorialStep);
}
