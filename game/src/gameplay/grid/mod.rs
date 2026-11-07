use crate::{
    gameplay::{
        layout::{GameLayout, LayoutElementPosition},
        piece::Piece,
    },
    prelude::*,
    screens::run_conditions::in_game,
};

use mplk_ecs::transform::TrackPosition;
use mplk_grid::{
    bit_grid::{BitGrid, BitGridMask},
    debug::DebugGridValue,
    error::GridTileError,
    nested_grid,
    prelude::*,
    tile::TileIterator,
};

pub mod spawn;

pub fn plugin(app: &mut App) {
    app.add_plugins(spawn::plugin);
    app.add_message::<GridSectionCompletedMsg>();
    app.add_systems(Update, IngredientGrid::track_position);
    app.add_systems(Update, clear_grid_section.run_if(in_game));
}

pub(crate) const INGREDIENT_GRID_SIZE: u16 = 9;
pub(crate) const INGREDIENT_TILE_SIZE: u16 = 64;
pub(crate) const INGREDIENT_TILE_SIZE_F32: f32 = INGREDIENT_TILE_SIZE as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridSection {
    Column(i16),
    Row(i16),
    Area(i16),
}

#[derive(Debug, Message)]
pub struct GridSectionCompletedMsg(pub GridSection);

pub type ResourceGridResult<T> = Result<T, GridTileError>;

#[derive(Component, Clone)]
#[require(Transform)]
pub struct IngredientGrid {
    grid: BitGrid,
    position: Vec2,
}
#[cfg_attr(not(test), expect(unused))]
impl IngredientGrid {
    pub const AREA_SIZE: u16 = 3;

    pub fn new(grid: BitGrid, position: Vec2) -> Option<Self> {
        let rem = grid.grid_size() % Self::AREA_SIZE;
        (rem == TileGridSize::ZERO).then_some(Self { grid, position })
    }

    pub fn preview_piece_placement(
        &self,
        piece: &Piece,
        tile: TileCoords,
    ) -> ResourceGridResult<Vec<GridSection>> {
        let mut preview_grid = self.clone();
        for t in &piece.tiles {
            let t = t + tile;
            preview_grid.grid.place_tile(t)?;
        }
        Ok(preview_grid.completed_sections(piece, tile))
    }

    pub fn place_piece(
        &mut self,
        piece: &Piece,
        tile: TileCoords,
    ) -> ResourceGridResult<Vec<GridSection>> {
        for t in &piece.tiles {
            let t = t + tile;
            self.grid.place_tile(t)?;
        }
        Ok(self.completed_sections(piece, tile))
    }

    pub fn place_tiles(
        &mut self,
        tiles: impl IntoIterator<Item = TileCoords>,
    ) -> ResourceGridResult<()> {
        let mask = self.tiles_to_mask(tiles)?;
        self.grid.occupied_mask += mask;
        Ok(())
    }

    fn completed_sections(&self, piece: &Piece, tile: TileCoords) -> Vec<GridSection> {
        let mut set = HashSet::new();
        for t in &piece.tiles {
            let t = t + tile;
            if self.row_done(t.y).is_ok_and(|done| done) {
                set.insert(GridSection::Row(t.y));
            }
            if self.col_done(t.x).is_ok_and(|done| done) {
                set.insert(GridSection::Column(t.x));
            }
            if let Ok(i) = self.tile_to_area_index(t)
                && self.area_done(i).is_ok_and(|done| done)
            {
                set.insert(GridSection::Area(i));
            }
        }
        set.into_iter().collect()
    }

    pub fn row_done(&self, row: i16) -> ResourceGridResult<bool> {
        self.grid
            .iter_row(row)
            .ok_or(GridTileError::OutOfBounds)
            .and_then(|tiles| self.rect_done(tiles))
    }

    pub fn col_done(&self, column: i16) -> ResourceGridResult<bool> {
        self.grid
            .iter_column(column)
            .ok_or(GridTileError::OutOfBounds)
            .and_then(|tiles| self.rect_done(tiles))
    }

    pub fn area_done(&self, idx: i16) -> ResourceGridResult<bool> {
        self.iter_area(idx)
            .ok_or(GridTileError::OutOfBounds)
            .and_then(|tiles| self.rect_done(tiles))
    }

    pub fn clear_row(&mut self, row: i16) -> ResourceGridResult<()> {
        self.grid
            .iter_row(row)
            .ok_or(GridTileError::OutOfBounds)
            .map(|tiles| self.clear_rect(tiles))?
    }

    pub fn clear_col(&mut self, column: i16) -> ResourceGridResult<()> {
        self.grid
            .iter_column(column)
            .ok_or(GridTileError::OutOfBounds)
            .map(|tiles| self.clear_rect(tiles))?
    }

    pub fn clear_area(&mut self, idx: i16) -> ResourceGridResult<()> {
        self.iter_area(idx)
            .ok_or(GridTileError::OutOfBounds)
            .map(|tiles| self.clear_rect(tiles))?
    }

    fn tiles_to_mask(
        &self,
        tiles: impl IntoIterator<Item = TileCoords>,
    ) -> ResourceGridResult<BitGridMask> {
        let mut mask = BitGridMask::default();
        for t in tiles {
            match self.grid.tile_to_idx(t) {
                Some(idx) => mask += idx,
                None => return Err(GridTileError::OutOfBounds),
            }
        }
        Ok(mask)
    }

    fn max_idx(&self) -> u16 {
        self.grid.grid_size().element_product() - 1
    }

    fn tile_to_area_index(&self, tile: TileCoords) -> ResourceGridResult<i16> {
        if !self.grid.within_bounds(tile) {
            return Err(GridTileError::OutOfBounds);
        }

        let area_coords = tile / Self::AREA_SIZE as i16;
        let area_count = self.area_count();
        Ok(area_coords.y * area_count.x as i16 + area_coords.x)
    }

    fn iter_area_from_tile(&self, tile: TileCoords) -> ResourceGridResult<TileIterator> {
        self.tile_to_area_index(tile)
            .map(|idx| self.iter_area(idx).expect("invalid area"))
    }

    fn area_count(&self) -> TileGridSize {
        self.grid.grid_size() / Self::AREA_SIZE
    }

    fn max_area_idx(&self) -> u16 {
        let area_count = self.area_count();
        area_count.element_product() - 1
    }

    pub fn area_idx_to_start_tile(&self, idx: i16) -> Option<TileCoords> {
        (idx >= 0 && idx <= self.max_area_idx() as _).then(|| {
            let area_count = self.area_count();
            let idx = idx as u16;
            let x = idx.rem_euclid(area_count.x) * Self::AREA_SIZE;
            let y = (idx / area_count.x) * Self::AREA_SIZE;
            TileCoords::new(x as _, y as _)
        })
    }

    fn iter_area(&self, idx: i16) -> Option<TileIterator> {
        self.area_idx_to_start_tile(idx).map(|start_tile| {
            TileIterator::from_start_tile(start_tile, TileGridSize::splat(Self::AREA_SIZE))
        })
    }

    fn rect_done(&self, tiles: impl IntoIterator<Item = TileCoords>) -> ResourceGridResult<bool> {
        let mask = self.tiles_to_mask(tiles)?.mask();
        Ok((self.grid.occupied_mask.mask() & mask) == mask)
    }

    fn clear_rect(
        &mut self,
        tiles: impl IntoIterator<Item = TileCoords>,
    ) -> ResourceGridResult<()> {
        let mask = self.tiles_to_mask(tiles)?;
        self.grid.occupied_mask -= mask;
        Ok(())
    }

    fn track_position(mut grid_q: Query<(&GlobalTransform, &mut IngredientGrid)>) {
        for (t, mut grid) in &mut grid_q {
            grid.position = t.translation().truncate();
        }
    }
}
impl LayoutElementPosition for IngredientGrid {
    fn position(layout: &GameLayout, grid: &IngredientGrid) -> Vec2 {
        layout.grid_position(grid)
    }
}

nested_grid!(IngredientGrid, (), grid);
impl TrackPosition for IngredientGrid {
    fn position(&self) -> Vec2 {
        self.position
    }
    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }
}
impl DebugGridValue for IngredientGrid {
    fn value_to_char(_value: &()) -> char {
        'X'
    }
}

fn clear_grid_section(
    mut section_mr: MessageReader<GridSectionCompletedMsg>,
    mut grid: Single<&mut IngredientGrid>,
) {
    // todo: tween out marked tiles
    for msg in section_mr.read() {
        match msg.0 {
            GridSection::Column(col) => or_continue!(grid.clear_col(col)),
            GridSection::Row(row) => or_continue!(grid.clear_row(row)),
            GridSection::Area(idx) => or_continue!(grid.clear_area(idx)),
        };
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::U16Vec2;
    use test_case::test_case;

    use mplk_grid::ext::ParsedGrid;
    use mplk_grid_test_utils::TestGridUtils;

    use super::*;

    #[test_case(vec![(0, 0), (1, 0), (2, 0)] => Ok(0b111))]
    #[test_case(vec![(0, 0), (1, 1), (2, 2)] => Ok(0b100000010000001))]
    #[test_case(vec![(100, 0)] => Err(GridTileError::OutOfBounds))]
    #[tracing_test::traced_test]
    fn tiles_to_mask(tiles: Vec<(i16, i16)>) -> ResourceGridResult<u128> {
        let grid = test_grid();

        let tiles = tiles
            .into_iter()
            .map(Into::into)
            .collect::<Vec<TileCoords>>();
        grid.tiles_to_mask(tiles).map(|m| m.mask())
    }

    #[test_case(vec![] => Ok(vec![(1, 1), (2, 1), (3, 1), (4, 1)]))]
    #[test_case(vec![(1, 1)] => Ok(vec![(1, 1), (2, 1), (3, 1), (4, 1)]))]
    #[test_case(vec![(0, 1), (5, 1)] => Ok(vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1)]))]
    #[test_case(vec![(100, 0)] => Err(GridTileError::OutOfBounds))]
    #[tracing_test::traced_test]
    fn place_tiles(extra_occupied_tiles: Vec<(i16, i16)>) -> ResourceGridResult<Vec<(i16, i16)>> {
        let mut grid = test_grid();
        grid.place_tiles(extra_occupied_tiles.into_iter().map(Into::into))?;

        Ok(grid.grid.iter_occupied_tiles().map(Into::into).collect())
    }

    #[test]
    #[tracing_test::traced_test]
    fn max_idx() {
        let grid = test_grid();

        assert_eq!(17, grid.max_idx())
    }

    #[test_case((-10, 0) => Err(GridTileError::OutOfBounds))]
    #[test_case((100, 100) => Err(GridTileError::OutOfBounds))]
    #[test_case((0, 0) => Ok(0))]
    #[test_case((3, 0) => Ok(1))]
    #[test_case((5, 2) => Ok(1))]
    #[tracing_test::traced_test]
    fn tile_to_area_index(tile: (i16, i16)) -> ResourceGridResult<i16> {
        let grid = test_grid();

        grid.tile_to_area_index(tile.into())
    }

    #[test_case(-10 => None)]
    #[test_case(100 => None)]
    #[test_case(0 => Some(vec![
        (0, 0), (1, 0), (2, 0),
        (0, 1), (1, 1), (2, 1),
        (0, 2), (1, 2), (2, 2),
    ]))]
    #[tracing_test::traced_test]
    fn iter_area(idx: i16) -> Option<Vec<(i16, i16)>> {
        let grid = test_grid();

        grid.iter_area(idx)
            .map(|tiles| tiles.map(|t| (t.x, t.y)).collect())
    }

    #[test_case((-10, 0) => Err(GridTileError::OutOfBounds))]
    #[test_case((100, 100) => Err(GridTileError::OutOfBounds))]
    #[test_case((0, 0) => Ok(vec![
        (0, 0), (1, 0), (2, 0),
        (0, 1), (1, 1), (2, 1),
        (0, 2), (1, 2), (2, 2),
    ]))]
    #[tracing_test::traced_test]
    fn iter_area_from_tile(tile: (i16, i16)) -> ResourceGridResult<Vec<(i16, i16)>> {
        let grid = test_grid();

        Ok(grid
            .iter_area_from_tile(tile.into())?
            .map(|t| (t.x, t.y))
            .collect())
    }

    #[test]
    #[tracing_test::traced_test]
    fn area_count() {
        let grid = test_grid();

        assert_eq!(U16Vec2::new(2, 1), grid.area_count())
    }

    #[test]
    #[tracing_test::traced_test]
    fn max_area_idx() {
        let grid = test_grid();

        assert_eq!(1, grid.max_area_idx())
    }

    #[test_case(-10 => None)]
    #[test_case(100 => None)]
    #[test_case(0 => Some((0, 0)))]
    #[test_case(1 => Some((3, 0)))]
    #[tracing_test::traced_test]
    fn area_idx_to_start_tile(idx: i16) -> Option<(i16, i16)> {
        let grid = test_grid();

        grid.area_idx_to_start_tile(idx).map(|t| (t.x, t.y))
    }

    #[test_case(1, vec![] => Ok(false))]
    #[test_case(1, vec![(0, 1)] => Ok(false))]
    #[test_case(1, vec![(0, 1), (5, 1)] => Ok(true))]
    #[test_case(100, vec![(0, 1)] => Err(GridTileError::OutOfBounds))]
    #[test_case(1, vec![(100, 0)] => Err(GridTileError::OutOfBounds))]
    #[tracing_test::traced_test]
    fn row_done(row: i16, extra_occupied_tiles: Vec<(i16, i16)>) -> ResourceGridResult<bool> {
        let mut grid = test_grid();
        grid.place_tiles(extra_occupied_tiles.into_iter().map(Into::into))?;

        grid.row_done(row)
    }

    #[test_case(1, vec![] => Ok(false))]
    #[test_case(1, vec![(1, 0)] => Ok(false))]
    #[test_case(1, vec![(1, 0), (1, 2)] => Ok(true))]
    #[test_case(100, vec![(0, 1)] => Err(GridTileError::OutOfBounds))]
    #[test_case(1, vec![(100, 0)] => Err(GridTileError::OutOfBounds))]
    #[tracing_test::traced_test]
    fn col_done(col: i16, extra_occupied_tiles: Vec<(i16, i16)>) -> ResourceGridResult<bool> {
        let mut grid = test_grid();
        grid.place_tiles(extra_occupied_tiles.into_iter().map(Into::into))?;

        grid.col_done(col)
    }

    #[test_case(0, vec![] => Ok(false))]
    #[test_case(0, vec![
        (0, 0), (1, 0), (2, 0),
        (0, 1),
        (0, 2), (1, 2), (2, 2)] => Ok(true))]
    #[test_case(100, vec![(0, 1)] => Err(GridTileError::OutOfBounds))]
    #[test_case(1, vec![(100, 0)] => Err(GridTileError::OutOfBounds))]
    #[tracing_test::traced_test]
    fn area_done(area: i16, extra_occupied_tiles: Vec<(i16, i16)>) -> ResourceGridResult<bool> {
        let mut grid = test_grid();
        grid.place_tiles(extra_occupied_tiles.into_iter().map(Into::into))?;

        grid.area_done(area)
    }

    #[test_case(0 => Ok(()))]
    #[test_case(1 => Ok(()))]
    #[test_case(100 => Err(GridTileError::OutOfBounds))]
    #[tracing_test::traced_test]
    fn clear_row(row: i16) -> ResourceGridResult<()> {
        let mut grid = test_grid();
        grid.clear_row(row)?;

        let row_clear = grid
            .grid
            .iter_row(row)
            .expect("invalid row")
            .all(|t| !grid.grid.occupied(t));
        assert!(row_clear);
        Ok(())
    }

    #[test_case(0 => Ok(()))]
    #[test_case(1 => Ok(()))]
    #[test_case(100 => Err(GridTileError::OutOfBounds))]
    #[tracing_test::traced_test]
    fn clear_col(col: i16) -> ResourceGridResult<()> {
        let mut grid = test_grid();
        grid.clear_col(col)?;

        let col_clear = grid
            .grid
            .iter_column(col)
            .expect("invalid col")
            .all(|t| !grid.grid.occupied(t));
        assert!(col_clear);
        Ok(())
    }

    #[test_case(0 => Ok(()))]
    #[test_case(1 => Ok(()))]
    #[test_case(100 => Err(GridTileError::OutOfBounds))]
    #[tracing_test::traced_test]
    fn clear_area(area: i16) -> ResourceGridResult<()> {
        let mut grid = test_grid();
        grid.clear_area(area)?;

        let area_clear = grid
            .iter_area(area)
            .expect("invalid area")
            .all(|t| !grid.grid.occupied(t));
        assert!(area_clear);
        Ok(())
    }

    fn test_grid() -> IngredientGrid {
        let parsed: ParsedGrid = TestGridUtils::TEST_LVL_6X3
            .parse()
            .expect("invalid test grid");
        let occupied_mask = parsed
            .occupied_tiles
            .iter()
            .fold(BitGridMask::default(), |acc, t| {
                acc + parsed.tile_to_idx(*t).expect("invalid tile")
            });

        let bit_grid = BitGrid::new(parsed.grid_size, TestGridUtils::TILE_SIZE)
            .with_occupied_mask(occupied_mask);
        IngredientGrid::new(bit_grid, Vec2::ZERO).expect("invalid test resource mask")
    }
}
