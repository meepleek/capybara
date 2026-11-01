use crate::prelude::*;
use bevy::prelude::*;

pub const DIRS_ORTHO_CW: [TileCoords; 4] = [
    TileCoords::NEG_Y,
    TileCoords::X,
    TileCoords::Y,
    TileCoords::NEG_X,
];
pub const DIRS_DIAG_CW: [TileCoords; 4] = [
    TileCoords::ONE,
    TileCoords::new(1, -1),
    TileCoords::NEG_ONE,
    TileCoords::new(-1, 1),
];
pub const DIRS_CW: [TileCoords; 8] = [
    TileCoords::NEG_Y,
    TileCoords::new(1, -1),
    TileCoords::X,
    TileCoords::ONE,
    TileCoords::Y,
    TileCoords::new(-1, 1),
    TileCoords::NEG_X,
    TileCoords::NEG_ONE,
];

pub enum NeighbourDirection {
    Orthogonal,
    Diagonal,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum TileDir {
    Ortho(TileOrthoDir),
    Diag(TileDiagDir),
}
impl TileDir {
    pub const NORTH: TileCoords = TileCoords::NEG_Y;
    pub const NORTH_EAST: TileCoords = TileCoords::new(1, -1);
    pub const EAST: TileCoords = TileCoords::X;
    pub const SOUTH_EAST: TileCoords = TileCoords::ONE;
    pub const SOUTH: TileCoords = TileCoords::Y;
    pub const SOUTH_WEST: TileCoords = TileCoords::new(-1, 1);
    pub const WEST: TileCoords = TileCoords::NEG_X;
    pub const NORTH_WEST: TileCoords = TileCoords::NEG_ONE;

    pub const DIRS: [TileDir; 8] = {
        use TileDiagDir::*;
        use TileDir::*;
        use TileOrthoDir::*;

        [
            Ortho(North),
            Diag(NorthEast),
            Ortho(East),
            Diag(SouthEast),
            Ortho(South),
            Diag(SouthWest),
            Ortho(West),
            Diag(NorthWest),
        ]
    };

    pub fn from_direction(dir: TileCoords) -> Option<Self> {
        match dir {
            Self::NORTH | Self::EAST | Self::SOUTH | Self::WEST => Some(Self::Ortho(
                TileOrthoDir::from_direction(dir).expect("Invalid ortho dir"),
            )),
            Self::NORTH_EAST | Self::SOUTH_EAST | Self::SOUTH_WEST | Self::NORTH_WEST => Some(
                Self::Diag(TileDiagDir::from_direction(dir).expect("Invalid diag dir")),
            ),
            _ => None,
        }
    }

    pub fn direction(&self) -> TileCoords {
        use TileDir::*;

        match self {
            Ortho(tile_ortho_dir) => tile_ortho_dir.direction(),
            Diag(tile_diag_dir) => tile_diag_dir.direction(),
        }
    }

    pub fn world_direction(&self) -> Vec2 {
        // flip the Y-axis
        (self.direction() * TileCoords::new(1, -1)).as_vec2()
    }

    pub fn rotation(&self) -> Rot2 {
        match self {
            Self::Ortho(tile_ortho_dir) => tile_ortho_dir.rotation(),
            Self::Diag(tile_diag_dir) => tile_diag_dir.rotation(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Hash, strum::EnumIter)]
pub enum TileOrthoDir {
    North,
    East,
    South,
    West,
}
impl TileOrthoDir {
    pub fn from_direction(dir: TileCoords) -> Option<Self> {
        use TileOrthoDir::*;

        match dir {
            TileDir::NORTH => Some(North),
            TileDir::EAST => Some(East),
            TileDir::SOUTH => Some(South),
            TileDir::WEST => Some(West),
            _ => None,
        }
    }

    pub fn direction(&self) -> TileCoords {
        use TileOrthoDir::*;

        match self {
            North => TileDir::NORTH,
            East => TileDir::EAST,
            South => TileDir::SOUTH,
            West => TileDir::WEST,
        }
    }

    pub fn rotation(&self) -> Rot2 {
        Rot2::degrees(match self {
            TileOrthoDir::North => 0.,
            TileOrthoDir::West => 90.,
            TileOrthoDir::South => 180.,
            TileOrthoDir::East => 270.,
        })
    }

    pub fn rotate_cw(&self) -> Self {
        use TileOrthoDir::*;

        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub fn rotate_ccw(&self) -> Self {
        use TileOrthoDir::*;

        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Hash, strum::EnumIter)]
pub enum TileDiagDir {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}
impl TileDiagDir {
    pub fn from_direction(dir: TileCoords) -> Option<Self> {
        use TileDiagDir::*;

        match dir {
            TileDir::NORTH_EAST => Some(NorthEast),
            TileDir::NORTH_WEST => Some(NorthWest),
            TileDir::SOUTH_EAST => Some(SouthEast),
            TileDir::SOUTH_WEST => Some(SouthWest),
            _ => None,
        }
    }

    pub fn direction(&self) -> TileCoords {
        use TileDiagDir::*;

        match self {
            NorthEast => TileDir::NORTH_EAST,
            SouthEast => TileDir::SOUTH_EAST,
            SouthWest => TileDir::SOUTH_WEST,
            NorthWest => TileDir::NORTH_WEST,
        }
    }

    pub fn rotation(&self) -> Rot2 {
        Rot2::degrees(match self {
            TileDiagDir::NorthWest => 45.,
            TileDiagDir::SouthWest => 135.,
            TileDiagDir::SouthEast => 225.,
            TileDiagDir::NorthEast => 315.,
        })
    }
}
