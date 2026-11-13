use bevy::color::palettes::tailwind;

use crate::{
    gameplay::{
        cauldron::Cauldron,
        grid::IngredientGrid,
        highlight::{DragHighlightPlugin, HighlightPalette},
        ingredient::Ingredient,
        piece::Piece,
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(HighlightPalette {
        draggable_hover: tailwind::AMBER_300.into(),
        draggable_active: tailwind::AMBER_500.into(),
        draggable_snap: tailwind::AMBER_800.into(),
        target_draggable_hover: tailwind::YELLOW_300.into(),
        target_draggable_active: tailwind::YELLOW_500.into(),
        target_draggable_snap: tailwind::YELLOW_800.into(),
    });
    app.add_plugins((
        DragHighlightPlugin::<Piece, IngredientGrid>::default(),
        DragHighlightPlugin::<Ingredient, Cauldron>::default(),
        DragHighlightPlugin::<Ingredient, Piece>::default(),
    ));
}
