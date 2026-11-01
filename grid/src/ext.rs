#[cfg(test)]
use crate::GridStorage;
use crate::{
    direction::{DIRS_CW, DIRS_DIAG_CW, DIRS_ORTHO_CW, NeighbourDirection},
    prelude::*,
    tile::{EMPTY_TILE_CHAR, OCCUPIED_TILE_CHAR, TileIterator},
};
use std::str::FromStr;

pub trait GridIterExt {
    fn iter_tiles(&self) -> TileIterator;
    fn iter_column(&self, column: i16) -> Option<TileIterator>;
    fn iter_row(&self, row: i16) -> Option<TileIterator>;
}

impl<TGrid: GridSize + GridBoundsExt> GridIterExt for TGrid {
    fn iter_tiles(&self) -> TileIterator {
        TileIterator::from_size(self.grid_size())
    }

    fn iter_column(&self, column: i16) -> Option<TileIterator> {
        self.col_within_bounds(column)
            .then(|| TileIterator::from_start_tile((column, 0), (1, self.grid_size().y)))
    }

    fn iter_row(&self, row: i16) -> Option<TileIterator> {
        self.row_within_bounds(row)
            .then(|| TileIterator::from_start_tile((0, row), (self.grid_size().x, 1)))
    }
}

pub trait GridNeighbourExt {
    fn occupied_neighbours(
        &self,
        tile: TileCoords,
        move_dir: NeighbourDirection,
        check_bounds: bool,
    ) -> impl Iterator<Item = TileCoords>;
    fn neighbour_dirs(neighbour_dir: NeighbourDirection) -> &'static [TileCoords];
}
impl<TGrid: GridBoundsExt> GridNeighbourExt for TGrid {
    fn occupied_neighbours(
        &self,
        tile: TileCoords,
        move_dir: NeighbourDirection,
        check_bounds: bool,
    ) -> impl Iterator<Item = TileCoords> {
        let dirs = Self::neighbour_dirs(move_dir);
        dirs.iter().copied().filter_map(move |dir| {
            let target = tile + dir;
            (!check_bounds || self.within_bounds(target)).then_some(target)
        })
    }

    fn neighbour_dirs(neighbour_dir: NeighbourDirection) -> &'static [TileCoords] {
        match neighbour_dir {
            NeighbourDirection::Orthogonal => &DIRS_ORTHO_CW,
            NeighbourDirection::Diagonal => &DIRS_DIAG_CW,
            NeighbourDirection::All => &DIRS_CW,
        }
    }
}

#[cfg(test)]
pub trait GridPrintExt<TValue> {
    fn print_ascii_debug_map(&self);
    fn print_ascii_debug_map_with_remap(
        &self,
        remap_occuption_tile: impl Fn(TileCoords, &TValue) -> Option<(char, DebugGridTileColor)>,
        remap_empty_tile: impl Fn(TileCoords) -> Option<(char, DebugGridTileColor)>,
    );
    fn ascii_debug_map(
        &self,
        remap_occuption_tile: impl Fn(TileCoords, &TValue) -> Option<(char, DebugGridTileColor)>,
        remap_empty_tile: impl Fn(TileCoords) -> Option<(char, DebugGridTileColor)>,
    ) -> String;
}
#[cfg(test)]
impl<TGrid: GridSize + GridStorage<TValue> + GridIterExt, TValue> GridPrintExt<TValue> for TGrid {
    fn print_ascii_debug_map(&self) {
        self.print_ascii_debug_map_with_remap(|_, _| None, |_| None);
    }

    fn print_ascii_debug_map_with_remap(
        &self,
        remap_occuption_tile: impl Fn(TileCoords, &TValue) -> Option<(char, DebugGridTileColor)>,
        remap_empty_tile: impl Fn(TileCoords) -> Option<(char, DebugGridTileColor)>,
    ) {
        println!(
            "{}",
            self.ascii_debug_map(remap_occuption_tile, remap_empty_tile)
        );
    }

    fn ascii_debug_map(
        &self,
        remap_occuption_tile: impl Fn(TileCoords, &TValue) -> Option<(char, DebugGridTileColor)>,
        remap_empty_tile: impl Fn(TileCoords) -> Option<(char, DebugGridTileColor)>,
    ) -> String {
        let size = self.grid_size();
        let mut dbg_map = String::with_capacity(size.element_product() as _);
        let x_axis = (0..self.grid_size().x)
            .map(|i| (i % 10).to_string())
            .collect::<String>();
        let header_style = DebugGridTileColor::Header.prefix();
        dbg_map.push_str(&format!("{header_style} _{}_ \n", x_axis));
        dbg_map.push_str(&format!("{header_style} 0"));
        let mut prev_y = 0;
        for tile in self.iter_tiles() {
            if tile.y != prev_y {
                prev_y = tile.y;
                dbg_map.push_str(&format!("{header_style}{} ", tile.y - 1));
                dbg_map.push('\n');
                dbg_map.push_str(&format!("{header_style}{:2}", tile.y));
            }
            let (c, col) = match self.get(tile) {
                Some(value) => {
                    remap_occuption_tile(tile, value).unwrap_or(('#', DebugGridTileColor::White))
                }
                None => remap_empty_tile(tile).unwrap_or(('.', DebugGridTileColor::White)),
            };
            dbg_map.push_str(&col.colored(c));
        }
        dbg_map.push_str(&format!(
            "{header_style}{} \n{header_style} _{}_ ",
            size.y - 1,
            x_axis
        ));
        dbg_map
    }
}

pub struct ParsedGrid {
    pub occupied_tiles: Vec<TileCoords>,
    pub grid_size: TileGridSize,
}
impl GridSize for ParsedGrid {
    fn grid_size(&self) -> TileGridSize {
        self.grid_size
    }
}
impl FromStr for ParsedGrid {
    type Err = String;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut size = TileCoords::ZERO;
        let mut errors = Vec::new();
        let mut occupied_tiles = Vec::with_capacity(s.len());

        for (tile, c) in s
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (TileCoords::new(x as i16, y as i16), c))
            })
        {
            size = size.max(tile);
            match c {
                OCCUPIED_TILE_CHAR => {
                    occupied_tiles.push(tile);
                }
                EMPTY_TILE_CHAR => {}
                _ => {
                    errors.push(format!("Invalid char '{c}' at {tile}"));
                }
            }
        }

        if errors.is_empty() {
            Ok(ParsedGrid {
                grid_size: (size + TileCoords::ONE).as_u16vec2(),
                occupied_tiles,
            })
        } else {
            Err(errors.join("\n"))
        }
    }
}
