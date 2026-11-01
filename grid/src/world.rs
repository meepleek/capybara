use crate::{direction::TileDir, prelude::*};
use bevy::prelude::*;
use mplk_ecs::transform::TrackPosition;

pub trait GridSize {
    fn grid_size(&self) -> TileGridSize;
}
pub trait GridTileSize {
    fn tile_size(&self) -> u16;
}
pub trait GridBoundsExt {
    fn row_within_bounds(&self, row: i16) -> bool;
    fn col_within_bounds(&self, column: i16) -> bool;
    fn within_bounds(&self, tile: impl Into<TileCoords>) -> bool;
}
pub trait GridTileIdxExt {
    fn tile_to_idx(&self, tile: impl Into<TileCoords>) -> Option<usize>;
}
pub trait GridWorldExt {
    fn world_size(&self) -> Vec2;
    fn world_to_tile(&self, pos: Vec2) -> Option<TileCoords>;
    fn world_to_board(&self, pos: Vec2) -> Vec2;
    fn world_to_tile_center(&self, pos: Vec2) -> Option<TileCoords>;
    fn tile_to_world(&self, tile: impl Into<TileCoords>) -> Option<Vec2>;
}
impl<TGrid: GridSize + GridTileSize + TrackPosition> GridWorldExt for TGrid {
    fn world_size(&self) -> Vec2 {
        self.grid_size().as_vec2() * self.tile_size() as f32
    }

    /// transform world position to board space (like screen space but in tiles)
    fn world_to_board(&self, pos: Vec2) -> Vec2 {
        let half_size = self.world_size() / 2.;
        let grid_pos = self.position();
        let x = half_size.x + pos.x - grid_pos.x;
        let y = half_size.y - pos.y + grid_pos.y;
        Vec2::new(x, y)
    }

    fn world_to_tile(&self, pos: Vec2) -> Option<TileCoords> {
        let board_pos = self.world_to_board(pos);
        let tile = (board_pos / self.tile_size() as f32).floor().as_i16vec2();
        if !self.within_bounds(tile) {
            return None;
        }
        Some(tile)
    }

    fn world_to_tile_center(&self, pos: Vec2) -> Option<TileCoords> {
        let tile = self.world_to_tile(pos)?;
        let board_pos = self.world_to_board(pos);
        let tile_size_f32 = self.tile_size() as f32;
        let tile_center =
            tile.as_vec2() * tile_size_f32 + TileDir::SOUTH_EAST.as_vec2() * (tile_size_f32 / 2.);
        let max = tile_size_f32 * 0.4;
        let centered = (tile_center - board_pos).abs().max_element() <= max;
        centered.then_some(tile)
    }

    fn tile_to_world(&self, tile: impl Into<TileCoords>) -> Option<Vec2> {
        let tile = tile.into();
        if !self.within_bounds(tile) {
            return None;
        }

        let half_size = self.world_size() / 2.;
        let half_tile = self.tile_size() as f32 / 2.;
        let tile_world = tile.as_vec2() * self.tile_size() as f32;
        let x = tile_world.x + half_tile - half_size.x;
        let y = -tile_world.y - half_tile + half_size.y;
        Some(Vec2::new(x, y) + self.position())
    }
}

impl<TGrid: GridSize> GridBoundsExt for TGrid {
    fn row_within_bounds(&self, row: i16) -> bool {
        row >= 0 && row < self.grid_size().y as _
    }

    fn col_within_bounds(&self, column: i16) -> bool {
        column >= 0 && column < self.grid_size().x as _
    }

    fn within_bounds(&self, tile: impl Into<TileCoords>) -> bool {
        let tile = tile.into();
        self.col_within_bounds(tile.x) && self.row_within_bounds(tile.y)
    }
}

impl<TGrid: GridSize> GridTileIdxExt for TGrid {
    fn tile_to_idx(&self, tile: impl Into<TileCoords>) -> Option<usize> {
        let tile = tile.into();
        self.within_bounds(tile).then(|| {
            let grid_size = self.grid_size();
            (tile.y * grid_size.x as i16 + tile.x) as _
        })
    }
}

// todo: update test with grid position (non-zero cases)
#[cfg(test)]
mod tests {
    use test_case::test_case;
    use tracing_test::traced_test;

    use super::*;

    #[derive(Default)]
    struct TestGrid {
        position: Vec2,
    }
    impl GridSize for TestGrid {
        fn grid_size(&self) -> TileGridSize {
            (3, 3).into()
        }
    }
    impl GridTileSize for TestGrid {
        fn tile_size(&self) -> u16 {
            TestGridUtils::TILE_SIZE
        }
    }
    impl TrackPosition for TestGrid {
        fn position(&self) -> Vec2 {
            self.position
        }

        fn set_position(&mut self, position: Vec2) {
            self.position = position
        }
    }

    #[test_case((0, 0) => true)]
    #[test_case((0, 2) => true)]
    #[test_case((2, 2) => true)]
    #[test_case((1, 1) => true)]
    #[test_case((3, 0) => false)]
    #[test_case((0, 3) => false)]
    #[test_case((-1, 0) => false)]
    #[test_case((0, -1) => false)]
    #[traced_test]
    fn within_bounds(tile: (i16, i16)) -> bool {
        TestGrid::default().within_bounds(tile)
    }

    #[test_case((0., 0.), (0., 0.) => Vec2::new(144., 144.))]
    #[test_case((50., 0.), (0., 0.) => Vec2::new(194., 144.))]
    #[test_case((-144., 0.), (0., 0.) => Vec2::new(0., 144.))]
    #[test_case((-144., 0.), (50., 50.) => Vec2::new(-50., 194.))]
    #[traced_test]
    fn world_to_board(world: (f32, f32), position: (f32, f32)) -> Vec2 {
        let grid = TestGrid {
            position: position.into(),
        };
        grid.world_to_board(world.into())
    }

    #[test_case((-144., 144.) => Some(TileCoords::new(0, 0)))]
    #[test_case((-100., 100.) => Some(TileCoords::new(0, 0)))]
    #[test_case((-96., 96.) => Some(TileCoords::new(0, 0)))]
    #[test_case((-48.1, 48.1) => Some(TileCoords::new(0, 0)))]
    #[test_case((-48., 48.) => Some(TileCoords::new(1, 1)))]
    #[test_case((0., 0.) => Some(TileCoords::new(1, 1)))]
    #[test_case((48., 0.) => Some(TileCoords::new(2, 1)))]
    #[test_case((95., -95.) => Some(TileCoords::new(2, 2)))]
    #[test_case((143.9, -143.9) => Some(TileCoords::new(2, 2)))]
    #[test_case((95., 144.) => Some(TileCoords::new(2, 0)))]
    #[test_case((-144., -143.9) => Some(TileCoords::new(0, 2)))]
    #[test_case((144., 0.) => None)]
    #[test_case((0., -144.) => None)]
    #[traced_test]
    fn world_to_tile(world: (f32, f32)) -> Option<TileCoords> {
        TestGrid::default().world_to_tile(world.into())
    }

    #[test_case((0., 0.), (0., 0.) => TileCoords::new(1, 1))]
    #[test_case((0., 0.), (50., 0.) => TileCoords::new(0, 1))]
    #[test_case((0., 0.), (50., 50.) => TileCoords::new(0, 2))]
    #[traced_test]
    fn world_to_tile_offcenter(world: (f32, f32), position: (f32, f32)) -> TileCoords {
        let grid = TestGrid {
            position: position.into(),
        };
        grid.world_to_tile(world.into())
            .expect("invalid world position")
    }

    #[test_case((0, 0) => Some(Vec2::new(-96., 96.)))]
    #[test_case((1, 1) => Some(Vec2::new(0., 0.)))]
    #[test_case((2, 2) => Some(Vec2::new(96., -96.)))]
    #[test_case((3, 0) => None)]
    #[test_case((0, 3) => None)]
    #[traced_test]
    fn tile_to_world(tile: (i16, i16)) -> Option<Vec2> {
        TestGrid::default().tile_to_world(tile)
    }

    #[test_case((0, 0), (0., 0.) => Vec2::new(-96., 96.))]
    #[test_case((0, 0), (50., 0.) => Vec2::new(-46., 96.))]
    #[test_case((0, 0), (50., 50.) => Vec2::new(-46., 146.))]
    #[traced_test]
    fn tile_to_world_offcenter(tile: (i16, i16), position: (f32, f32)) -> Vec2 {
        let grid = TestGrid {
            position: position.into(),
        };
        grid.tile_to_world(tile).expect("invalid tile")
    }

    #[test_case(0, 0)]
    #[test_case(0, 1)]
    #[test_case(1, 0)]
    #[test_case(1, 1)]
    #[test_case(2, 2)]
    #[traced_test]
    fn tile_to_world_to_tile(tile_x: i16, tile_y: i16) {
        let expected_tile = TileCoords::new(tile_x, tile_y);
        let grid = TestGrid::default();
        let world_pos = grid.tile_to_world(expected_tile).expect("valid world pos");
        let tile = grid.world_to_tile(world_pos).expect("valid tile");
        pretty_assertions::assert_eq!(expected_tile, tile);
    }

    #[test_case((-144., 144.) => None)]
    #[test_case((-100., 100.) => Some(TileCoords::new(0, 0)))]
    #[test_case((-96., 96.) => Some(TileCoords::new(0, 0)))]
    #[test_case((-48.1, 48.1) => None)]
    #[test_case((-48., 48.) => None)]
    #[test_case((0., 0.) => Some(TileCoords::new(1, 1)))]
    #[test_case((48., 0.) => None)]
    #[test_case((95., -95.) => Some(TileCoords::new(2, 2)))]
    #[test_case((143.9, -143.9) => None)]
    #[test_case((95., 144.) => None)]
    #[test_case((-144., -143.9) => None)]
    #[test_case((144., 0.) => None)]
    #[test_case((0., -144.) => None)]
    #[traced_test]
    fn world_to_tile_center(world: (f32, f32)) -> Option<TileCoords> {
        TestGrid::default().world_to_tile_center(world.into())
    }

    #[test_case((0., 0.), (0., 0.) => TileCoords::new(1, 1))]
    #[test_case((0., 0.), (96., 0.) => TileCoords::new(0, 1))]
    #[test_case((0., 0.), (96., 96.) => TileCoords::new(0, 2))]
    #[traced_test]
    fn world_to_tile_center_offcenter(world: (f32, f32), grid_position: (f32, f32)) -> TileCoords {
        let grid = TestGrid {
            position: grid_position.into(),
        };
        grid.world_to_tile_center(world.into())
            .expect("invalid world position")
    }

    #[test_case((0, 0), (0., 0.))]
    #[test_case((1, 1), (0., 0.))]
    #[test_case((2, 2), (0., 0.))]
    #[test_case((1, 1), (50., 0.))]
    #[test_case((1, 1), (0., 50.))]
    #[test_case((1, 1), (50., 50.))]
    #[traced_test]
    fn tile_to_world_to_tile_center(tile: (i16, i16), grid_position: (f32, f32)) {
        let grid = TestGrid {
            position: grid_position.into(),
        };
        let expected_tile: TileCoords = tile.into();
        let world_pos = grid.tile_to_world(expected_tile).expect("valid world pos");
        let center = grid.world_to_tile_center(world_pos).expect("valid center");
        pretty_assertions::assert_eq!(expected_tile, center);
    }
}
