use crate::{GridStorage, error::PlaceError, prelude::*};
use bevy::prelude::*;
use std::ops;

#[derive(Debug, Clone, Copy, Deref, DerefMut, Default)]
pub struct BitGridMask(u128);
impl BitGridMask {
    pub fn new(mask: u128) -> Self {
        Self(mask)
    }

    pub fn mask(&self) -> u128 {
        self.0
    }

    pub fn from_idx(idx: usize) -> Self {
        let mut mask = Self::default();
        mask.add_by_idx(idx);
        mask
    }

    pub fn add_by_idx(&mut self, idx: usize) {
        self.0 |= 1 << idx
    }
}

impl ops::Add<BitGridMask> for BitGridMask {
    type Output = BitGridMask;

    fn add(self, rhs: BitGridMask) -> Self::Output {
        #[expect(clippy::suspicious_arithmetic_impl)]
        Self::new(self.0 | rhs.0)
    }
}
impl ops::AddAssign<BitGridMask> for BitGridMask {
    fn add_assign(&mut self, rhs: BitGridMask) {
        *self = *self + rhs
    }
}
impl ops::Sub<BitGridMask> for BitGridMask {
    type Output = BitGridMask;

    fn sub(self, rhs: BitGridMask) -> Self::Output {
        Self::new(self.0 & !rhs.0)
    }
}
impl ops::SubAssign<BitGridMask> for BitGridMask {
    fn sub_assign(&mut self, rhs: BitGridMask) {
        *self = *self - rhs
    }
}
impl ops::Add<usize> for BitGridMask {
    type Output = BitGridMask;

    fn add(self, idx: usize) -> Self::Output {
        self + Self::new(1 << idx)
    }
}
impl ops::AddAssign<usize> for BitGridMask {
    fn add_assign(&mut self, idx: usize) {
        *self = *self + idx
    }
}
impl ops::Sub<usize> for BitGridMask {
    type Output = BitGridMask;

    fn sub(self, idx: usize) -> Self::Output {
        self - Self::new(1 << idx)
    }
}
impl ops::SubAssign<usize> for BitGridMask {
    fn sub_assign(&mut self, idx: usize) {
        *self = *self - idx
    }
}

pub struct BitGrid {
    grid_size: TileGridSize,
    tile_size: u16,
    pub occupied_mask: BitGridMask,
    max_idx: usize,
}
impl BitGrid {
    pub fn new(grid_size: impl Into<TileGridSize>, tile_size: u16) -> Self {
        let grid_size = grid_size.into();
        if grid_size.min_element() == 0 {
            panic!("Invalid dimensions - no dimension can be 0");
        }
        if grid_size.element_product() as u32 > u128::BITS {
            panic!("Invalid dimensions - grid area can't exceed bit count");
        }

        Self {
            grid_size,
            tile_size,
            occupied_mask: BitGridMask::default(),
            max_idx: (grid_size.element_product() - 1) as _,
        }
    }

    pub fn with_occupied_mask(mut self, mask: BitGridMask) -> Self {
        self.occupied_mask = mask;
        self
    }

    pub fn idx_within_bounds(&self, idx: usize) -> bool {
        idx <= self.max_idx
    }

    pub fn can_place_at(&self, tile: TileCoords) -> Result<(), PlaceError> {
        if !self.within_bounds(tile) {
            return Err(PlaceError::OutOfBounds);
        } else if self.occupied(tile) {
            return Err(PlaceError::Taken);
        }
        Ok(())
    }

    pub fn place_tile(&mut self, tile: impl Into<TileCoords>) -> Result<(), PlaceError> {
        let tile = tile.into();
        self.can_place_at(tile)?;
        let idx = self.tile_to_idx(tile).expect("valid idx");
        self.occupied_mask += idx;

        Ok(())
    }

    pub fn clear_tile(&mut self, tile: TileCoords) -> bool {
        if self.occupied(tile) {
            self.occupied_mask -= self.tile_to_idx(tile).expect("valid idx");
            true
        } else {
            false
        }
    }

    pub fn iter_occupied_tiles(&self) -> impl Iterator<Item = TileCoords> {
        self.iter_tiles().filter(|t| self.occupied(*t))
    }
}

impl GridSize for BitGrid {
    fn grid_size(&self) -> TileGridSize {
        self.grid_size
    }
}
impl GridTileSize for BitGrid {
    fn tile_size(&self) -> u16 {
        self.tile_size
    }
}
impl GridStorage<()> for BitGrid {
    fn occupied(&self, tile: TileCoords) -> bool {
        self.tile_to_idx(tile).is_some_and(|idx| {
            let tile_mask = 1 << idx;
            (self.occupied_mask.0 & tile_mask) != 0
        })
    }

    fn get(&self, tile: TileCoords) -> Option<&()> {
        self.occupied(tile).then_some(&())
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::ext::ParsedGrid;

    use super::*;

    #[test_case((1, 1))]
    #[test_case((3, 6))]
    #[test_case((12, 10))]
    #[test_case((0, 0) => panics)]
    #[test_case((0, 3) => panics)]
    #[test_case((3, 0) => panics)]
    // "grid too large for a bitmask"
    #[test_case((12, 12) => panics )]
    fn new(grid_size: (u16, u16)) {
        let grid = BitGrid::new(grid_size, TestGridUtils::TILE_SIZE);

        assert_eq!(0, grid.occupied_mask.0);
    }

    #[test_case(1, 2 => matches Ok(_))]
    #[test_case(4, 2 => matches Ok(_))]
    #[test_case(2, 1 => matches Err(PlaceError::Taken))]
    #[test_case(1, 1 => matches Err(PlaceError::Taken))]
    #[test_case(6, 0 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(0, 3 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(50, 0 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(0, 50 => matches Err(PlaceError::OutOfBounds))]
    fn can_place_at_coords(x: i16, y: i16) -> Result<(), PlaceError> {
        let grid = test_grid();
        grid.can_place_at((x, y).into())
    }

    #[test]
    fn cannot_place_at_coords_when_taken() {
        let coords: TileCoords = (0, 0).into();
        let mut grid = test_grid();
        grid.place_tile(coords).expect("Place first piece");

        assert_eq!(grid.can_place_at(coords), Err(PlaceError::Taken));
    }

    fn test_grid() -> BitGrid {
        let parsed: ParsedGrid = TestGridUtils::TEST_LVL_6X3
            .parse()
            .expect("invalid test grid");
        let occupied_mask = parsed
            .occupied_tiles
            .iter()
            .fold(BitGridMask::default(), |acc, t| {
                acc + parsed.tile_to_idx(*t).expect("invalid tile")
            });

        BitGrid::new(parsed.grid_size, TestGridUtils::TILE_SIZE).with_occupied_mask(occupied_mask)
    }
}
