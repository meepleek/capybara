pub use crate::prelude::*;
use bevy::math::U16Vec2;

pub mod bit_grid;
pub mod debug;
pub mod direction;
pub mod error;
pub mod ext;
pub mod prelude;
pub mod tile;
pub mod value_grid;
pub mod world;

#[cfg(test)]
pub mod test_utils;

pub type TileGridSize = U16Vec2;

pub trait GridStorage<TValue> {
    fn occupied(&self, tile: TileCoords) -> bool;
    fn get(&self, tile: TileCoords) -> Option<&TValue>;
}

#[macro_export]
macro_rules! nested_grid {
    ($grid_ty: ty, $value_ty: ty, $grid_fld: ident) => {
        impl GridSize for $grid_ty {
            fn grid_size(&self) -> TileGridSize {
                self.$grid_fld.grid_size()
            }
        }
        impl GridTileSize for $grid_ty {
            fn tile_size(&self) -> u16 {
                self.$grid_fld.tile_size()
            }
        }
        impl gameplay::grid::GridStorage<$value_ty> for $grid_ty {
            fn occupied(&self, tile: TileCoords) -> bool {
                self.$grid_fld.occupied(tile)
            }

            fn get(&self, tile: TileCoords) -> Option<&$value_ty> {
                self.$grid_fld.get(tile)
            }
        }
    };
}
