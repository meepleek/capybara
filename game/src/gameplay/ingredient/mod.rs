use crate::gameplay::{
    grid::{GridSection, INGREDIENT_TILE_SIZE_F32},
    inventory::InventoryIndex,
    layout::{GameLayout, InventoryPosition},
};
pub use crate::prelude::*;

mod drag;
mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((spawn::plugin, drag::plugin));
}

pub(crate) const INGREDIENT_MAX: u8 = 6;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ingredient {
    Honey,
    HoneyPlus,
    Apple,
    Pineapple,
    Ginger,
    Jalapeno,
    Orange,
    Lime,
    Air,
    AirPlus,
    Coffee,
    Chocolate,
    Water,
    Ice,
}
impl Ingredient {
    pub fn spritesheet_idx(&self) -> usize {
        match self {
            Ingredient::Honey => 8,
            Ingredient::HoneyPlus => 16,
            Ingredient::Apple => 9,
            Ingredient::Pineapple => 17,
            Ingredient::Ginger => 10,
            Ingredient::Jalapeno => 18,
            Ingredient::Orange => 11,
            Ingredient::Lime => 19,
            Ingredient::Air => 12,
            Ingredient::AirPlus => 20,
            Ingredient::Coffee => 13,
            Ingredient::Chocolate => 21,
            Ingredient::Water => 14,
            Ingredient::Ice => 22,
        }
    }

    pub fn icon_scene(self, translation: Vec3) -> impl Scene {
        let index = self.spritesheet_idx();
        bsn! {
            #ingredient_src_icon
            Transform { translation }
            from_collection(move |sprites: &SpriteAssets| Sprite {
                image: sprites.ingredients_sheet.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: sprites.ingredients_layout.clone(),
                    index
                }),
                custom_size: Some(Vec2::splat(INGREDIENT_TILE_SIZE_F32)),
                ..default()
            })
        }
    }

    pub fn section_ingredient(section: GridSection) -> Ingredient {
        use GridSection::*;
        use Ingredient::*;

        match section {
            Column(0..=2) | Row(6..=8) | Area(1) => Apple,
            Column(3..=5) | Row(3..=5) => Orange,
            Column(6..=8) | Row(0..=2) => Honey,
            Area(0) | Area(8) => Coffee,
            Area(2) | Area(4) | Area(6) => Water,
            Area(3) | Area(5) => Air,
            Area(7) => Ginger,
            _ => unreachable!("invalid grid section"),
        }
    }
}
impl InventoryPosition for Ingredient {
    fn inventory_position(layout: &GameLayout, idx: &InventoryIndex) -> Vec2 {
        layout.ingredient_position(idx.0)
    }
}
