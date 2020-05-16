use sdl2::rect::Rect;

const PLATFORM_TILE_DIMENSION: u8 = 64;
const CHARACTER_TILE_WIDTH: u8 = 96;
const CHARACTER_TILE_HEIGHT: u8 = 128;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum TileSheet {
    Character,
    Enemy,
    Platform,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Tile {
    Character { tile: CharacterTile },
    Enemy { tile: EnemyTile },
    Platform { tile: PlatformTile },
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum CharacterTile {
    Jump,
    Slide,
    Run1,
    Run2,
    Run3,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum EnemyTile {
    BatFly1,
    BatFly2,
    BeeFly1,
    BeeFly2,
    BugRun1,
    BugRun2,
    MouseRun1,
    MouseRun2,
    SpiderRun1,
    SpiderRun2,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum PlatformTile {
    GrassyGround,
    Ground,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct TileData {
    pub tile: Tile,
    pub tile_sheet: TileSheet,
    pub bounds_in_tile_sheet: Rect,
}

/// I am hoping that since `TileData` is not heap allocated, this is
/// cheap enough to be called every single tick if required
pub fn build_tile_data(tile: Tile) -> TileData {
    let tile_sheet: TileSheet;
    let bounds_in_tile_sheet: Rect;

    match tile {
        Tile::Character { tile } => {
            tile_sheet = TileSheet::Character;
            bounds_in_tile_sheet = match tile {
                CharacterTile::Jump => build_char_bounds(TilePos { row: 0, col: 1 }),
                CharacterTile::Slide => build_char_bounds(TilePos { row: 1, col: 1 }),
                CharacterTile::Run1 => build_char_bounds(TilePos { row: 2, col: 6 }),
                CharacterTile::Run2 => build_char_bounds(TilePos { row: 2, col: 7 }),
                CharacterTile::Run3 => build_char_bounds(TilePos { row: 2, col: 8 }),
            }
        }

        Tile::Enemy { tile } => {
            tile_sheet = TileSheet::Enemy;
            bounds_in_tile_sheet = match tile {
                EnemyTile::BatFly1 => Rect::new(71, 235, 70, 47),
                EnemyTile::BatFly2 => Rect::new(0, 0, 88, 37),
                EnemyTile::BeeFly1 => Rect::new(315, 353, 56, 48),
                EnemyTile::BeeFly2 => Rect::new(140, 23, 61, 42),
                EnemyTile::BugRun1 => Rect::new(141, 248, 58, 34),
                EnemyTile::BugRun2 => Rect::new(135, 99, 61, 34),
                EnemyTile::MouseRun1 => Rect::new(197, 475, 59, 35),
                EnemyTile::MouseRun2 => Rect::new(256, 475, 58, 35),
                EnemyTile::SpiderRun1 => Rect::new(0, 90, 72, 51),
                EnemyTile::SpiderRun2 => Rect::new(0, 37, 77, 53),
            }
        }

        Tile::Platform { tile } => {
            tile_sheet = TileSheet::Platform;
            bounds_in_tile_sheet = match tile {
                PlatformTile::GrassyGround => build_env_bounds(TilePos { row: 0, col: 0 }),
                PlatformTile::Ground => build_env_bounds(TilePos { row: 0, col: 3 }),
            }
        }
    };

    TileData {
        tile,
        tile_sheet,
        bounds_in_tile_sheet,
    }
}

struct TilePos {
    row: u8,
    col: u8,
}

fn build_char_bounds(tile_pos: TilePos) -> Rect {
    build_bounds(tile_pos, CHARACTER_TILE_WIDTH, CHARACTER_TILE_HEIGHT)
}

fn build_env_bounds(tile_pos: TilePos) -> Rect {
    build_bounds(tile_pos, PLATFORM_TILE_DIMENSION, PLATFORM_TILE_DIMENSION)
}

fn build_bounds(tile_pos: TilePos, tile_width: u8, tile_height: u8) -> Rect {
    // `from` here is for preventing overflowing on multiplying
    Rect::new(
        i32::from(tile_pos.col) * i32::from(tile_width),
        i32::from(tile_pos.row) * i32::from(tile_height),
        tile_width.into(),
        tile_height.into(),
    )
}
