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

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum CommonIngredient {
    #[default]
    Honey,
    Apple,
    Ginger,
    Orange,
    Air,
    Coffee,
    Water,
}
impl CommonIngredient {
    pub fn spritesheet_idx(&self) -> usize {
        match self {
            CommonIngredient::Honey => 8,
            CommonIngredient::Apple => 9,
            CommonIngredient::Ginger => 10,
            CommonIngredient::Orange => 11,
            CommonIngredient::Air => 12,
            CommonIngredient::Coffee => 13,
            CommonIngredient::Water => 14,
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

    pub fn section_ingredient(section: GridSection) -> CommonIngredient {
        use CommonIngredient::*;
        use GridSection::*;

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

    pub fn fuse(self) -> FusedIngredient {
        use CommonIngredient::*;
        use FusedIngredient::*;

        match self {
            Apple => Pineapple,
            Orange => Lime,
            Water => Ice,
            Honey => HoneyPlus,
            Ginger => Jalapeno,
            Air => AirPlus,
            Coffee => Chocolate,
        }
    }
}
impl InventoryPosition for CommonIngredient {
    fn inventory_position(layout: &GameLayout, idx: &InventoryIndex) -> Vec2 {
        layout.ingredient_position(idx.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum FusedIngredient {
    #[default]
    Pineapple,
    Lime,
    Ice,
    HoneyPlus,
    Jalapeno,
    AirPlus,
    Chocolate,
}
impl FusedIngredient {
    pub fn downgrade(self) -> CommonIngredient {
        use CommonIngredient::*;
        use FusedIngredient::*;

        match self {
            Pineapple => Apple,
            Lime => Orange,
            Ice => Water,
            HoneyPlus => Honey,
            Jalapeno => Ginger,
            AirPlus => Air,
            Chocolate => Coffee,
        }
    }

    pub fn spritesheet_idx(&self) -> usize {
        self.downgrade().spritesheet_idx() + 8 // next row down in the sheet
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ingredient {
    Common(CommonIngredient),
    Fused(FusedIngredient),
}
impl Default for Ingredient {
    fn default() -> Self {
        Self::Common(CommonIngredient::Apple)
    }
}
