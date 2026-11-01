use crate::{GridStorage, error::PlaceError, prelude::*};
use bevy::platform::collections::HashMap;

pub struct ValueGrid<TValue = ()> {
    grid_size: TileGridSize,
    tile_size: u16,
    occupied_tiles: HashMap<TileCoords, TValue>,
}
impl<TValue> ValueGrid<TValue> {
    pub fn new(grid_size: impl Into<TileGridSize>, tile_size: u16) -> Self {
        let grid_size = grid_size.into();
        if grid_size.min_element() == 0 {
            panic!("Invalid dimensions - no dimension can be 0");
        }

        Self {
            grid_size,
            tile_size,
            occupied_tiles: HashMap::default(),
        }
    }

    pub fn with_occupied_tiles(mut self, occupied_tiles: HashMap<TileCoords, TValue>) -> Self {
        self.occupied_tiles = occupied_tiles;
        self
    }

    pub fn can_place_at(&self, tile: TileCoords) -> Result<(), PlaceError> {
        if !self.within_bounds(tile) {
            return Err(PlaceError::OutOfBounds);
        } else if self.occupied(tile) {
            return Err(PlaceError::Taken);
        }
        Ok(())
    }

    pub fn place_tile(&mut self, tile: TileCoords, value: TValue) -> Result<(), PlaceError> {
        self.can_place_at(tile)?;
        self.occupied_tiles.insert(tile, value);

        Ok(())
    }

    pub fn get_tile_value_mut(&mut self, tile: TileCoords) -> Option<&mut TValue> {
        self.occupied_tiles.get_mut(&tile)
    }

    pub fn clear_tile(&mut self, tile: TileCoords) -> Option<TValue> {
        self.occupied_tiles.remove(&tile)
    }

    pub fn iter_occupied_tiles(&self) -> impl Iterator<Item = (TileCoords, &TValue)> {
        self.iter_tiles()
            .filter_map(|t| self.occupied_tiles.get(&t).map(|to| (t, to)))
    }
}
impl<TValue> GridSize for ValueGrid<TValue> {
    fn grid_size(&self) -> TileGridSize {
        self.grid_size
    }
}
impl<TValue> GridTileSize for ValueGrid<TValue> {
    fn tile_size(&self) -> u16 {
        self.tile_size
    }
}
impl<TValue> GridStorage<TValue> for ValueGrid<TValue> {
    fn occupied(&self, tile: TileCoords) -> bool {
        self.occupied_tiles.contains_key(&tile)
    }

    fn get(&self, tile: TileCoords) -> Option<&TValue> {
        self.occupied_tiles.get(&tile)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::ext::ParsedGrid;

    use super::*;

    #[test_case(1, 2 => matches Ok(_))]
    #[test_case(4, 2 => matches Ok(_))]
    #[test_case(2, 1 => matches Err(PlaceError::Taken))]
    #[test_case(1, 1 => matches Err(PlaceError::Taken))]
    #[test_case(6, 0 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(0, 3 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(50, 0 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(0, 50 => matches Err(PlaceError::OutOfBounds))]
    fn can_place_at_coords(x: i16, y: i16) -> Result<(), PlaceError> {
        let board = test_grid();
        board.can_place_at((x, y).into())
    }

    #[test]
    fn cannot_place_at_coords_when_taken() {
        let coords: TileCoords = (0, 0).into();
        let mut board = test_grid();
        board.place_tile(coords, ()).expect("Place first piece");

        assert_eq!(board.can_place_at(coords), Err(PlaceError::Taken));
    }

    fn test_grid() -> ValueGrid<()> {
        let parsed: ParsedGrid = TestGridUtils::TEST_LVL_6X3
            .parse()
            .expect("invalid test grid");
        ValueGrid::<()> {
            grid_size: parsed.grid_size,
            tile_size: TestGridUtils::TILE_SIZE,
            occupied_tiles: parsed.occupied_tiles.into_iter().map(|t| (t, ())).collect(),
        }
    }
}
