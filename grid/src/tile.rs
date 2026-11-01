use crate::prelude::*;
use bevy::math::I16Vec2;
use bevy::prelude::*;

pub const TILE_ALPHA_INACTIVE: f32 = 0.15;
pub const TILE_ALPHA_TARGETABLE: f32 = 1.0;
pub const TILE_ALPHA_HIDDEN: f32 = 0.0;
pub const EMPTY_TILE_CHAR: char = '.';
pub const OCCUPIED_TILE_CHAR: char = '#';

pub type TileCoords = I16Vec2;

pub trait CoordsExt {
    fn line_to(self, end: TileCoords) -> impl Iterator<Item = TileCoords>;
}
impl CoordsExt for TileCoords {
    fn line_to(self, end: TileCoords) -> impl Iterator<Item = TileCoords> {
        let delta = end - self;
        let len = self.chebyshev_distance(end);
        (0..=len).map(move |step| {
            let t = step as f32 / len as f32;
            let x = t * delta.x as f32 + self.x as f32;
            let y = t * delta.y as f32 + self.y as f32;
            TileCoords::new(x.round() as _, y.round() as _)
        })
    }
}

pub trait TileEvent {
    fn tile(&self) -> TileCoords;
}

#[derive(Component, Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct GridTileCoords(pub TileCoords);

pub struct TileIterator {
    grid_size: TileGridSize,
    tile: TileCoords,
    start_tile: TileCoords,
}
impl Iterator for TileIterator {
    type Item = TileCoords;

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.grid_size.as_i16vec2();
        if self.tile.y - self.start_tile.y >= size.y {
            None
        } else {
            let next = self.tile;
            self.tile.x += 1;
            if self.tile.x - self.start_tile.x == size.x {
                self.tile = (self.start_tile.x, self.tile.y + 1).into();
            }
            Some(next)
        }
    }
}
impl TileIterator {
    pub fn from_size(grid_size: impl Into<TileGridSize>) -> Self {
        Self::from_start_tile(TileCoords::ZERO, grid_size)
    }

    pub fn centered(grid_size: impl Into<TileGridSize>) -> Self {
        let grid_size = grid_size.into();
        Self::from_start_tile(-grid_size.as_i16vec2() / 2, grid_size)
    }

    pub fn from_start_tile(
        start_tile: impl Into<TileCoords>,
        grid_size: impl Into<TileGridSize>,
    ) -> Self {
        let tile = start_tile.into();
        Self {
            grid_size: grid_size.into(),
            tile,
            start_tile: tile,
        }
    }

    pub fn empty() -> Self {
        Self::from_start_tile(TileCoords::ZERO, (0, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case::test_case(
        (0, 1),
        vec![(0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)]
    )]
    #[test_case::test_case(
        (2, 2),
        vec![(2, 2), (3, 2), (4, 2), (2, 3), (3, 3), (4, 3)]
    )]
    fn from_start_tile(start_tile: (i16, i16), expected: Vec<(i16, i16)>) {
        let tiles: Vec<_> = TileIterator::from_start_tile(start_tile, (3, 2)).collect();
        assert_eq!(
            tiles,
            expected.into_iter().map(Into::into).collect::<Vec<_>>()
        );
    }

    #[test]
    fn from_size() {
        let tiles: Vec<_> = TileIterator::from_size((5, 3)).collect();
        assert_eq!(
            tiles,
            [
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (0, 1),
                (1, 1),
                (2, 1),
                (3, 1),
                (4, 1),
                (0, 2),
                (1, 2),
                (2, 2),
                (3, 2),
                (4, 2),
            ]
            .map(Into::into)
        );
    }

    #[test]
    fn centered() {
        let tiles: Vec<_> = TileIterator::centered((5, 3)).collect();
        assert_eq!(
            tiles,
            [
                (-2, -1),
                (-1, -1),
                (0, -1),
                (1, -1),
                (2, -1),
                (-2, 0),
                (-1, 0),
                (0, 0),
                (1, 0),
                (2, 0),
                (-2, 1),
                (-1, 1),
                (0, 1),
                (1, 1),
                (2, 1),
            ]
            .map(Into::into)
        );
    }

    #[test_case((0, 0), (0, 0), vec![(0, 0)])]
    #[test_case((0, 0), (1, 0), vec![(0, 0), (1, 0)])]
    #[test_case((0, 0), (4, 1), vec![(0, 0), (1, 0), (2, 1), (3, 1), (4, 1)])]
    fn line_to(start: (i16, i16), end: (i16, i16), expected: Vec<(i16, i16)>) {
        let actual: Vec<_> = TileCoords::from(start).line_to(end.into()).collect();

        pretty_assertions::assert_eq!(
            expected
                .into_iter()
                .map(Into::into)
                .collect::<Vec<TileCoords>>(),
            actual
        );
    }
}
