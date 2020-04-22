use sdl2::rect::Rect;

const ENVIRONMENT_TILE_DIMENSION: u8 = 64;
const CHARACTER_TILE_WIDTH: u8 = 80;
const CHARACTER_TILE_HEIGHT: u8 = 110;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum TileSheet {
    Character,
    Environment,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Tile {
    Character { tile: CharacterTile },
    Environment { tile: EnvironmentTile },
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum CharacterTile {
    Jump,
    Slide,
    Still,
    Walk1,
    Walk2,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum EnvironmentTile {
    GrassyGround,
    Ground,
    Plant,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct TileData {
    tile: Tile,
    tile_sheet: TileSheet,
    bounds_in_tile_sheet: Rect,
}

// I am hoping that since `TileData` is not heap allocated, this is
// cheap enough to be called every single frame if required
pub fn build_tile_data(tile: Tile) -> TileData {
    let tile_sheet: TileSheet;
    let bounds_in_tile_sheet: Rect;

    match tile {
        Tile::Character { tile } => {
            tile_sheet = TileSheet::Character;
            bounds_in_tile_sheet = match tile {
                CharacterTile::Jump => build_char_bounds(TilePos { row: 0, col: 1 }),
                CharacterTile::Slide => build_char_bounds(TilePos { row: 2, col: 1 }),
                CharacterTile::Still => build_char_bounds(TilePos { row: 0, col: 0 }),
                CharacterTile::Walk1 => build_char_bounds(TilePos { row: 1, col: 0 }),
                CharacterTile::Walk2 => build_char_bounds(TilePos { row: 1, col: 1 }),
            }
        }

        Tile::Environment { tile } => {
            tile_sheet = TileSheet::Environment;
            bounds_in_tile_sheet = match tile {
                EnvironmentTile::GrassyGround => build_env_bounds(TilePos { row: 0, col: 0 }),
                EnvironmentTile::Ground => build_env_bounds(TilePos { row: 0, col: 3 }),
                EnvironmentTile::Plant => build_env_bounds(TilePos { row: 5, col: 2 }),
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
    build_bounds(
        tile_pos,
        ENVIRONMENT_TILE_DIMENSION,
        ENVIRONMENT_TILE_DIMENSION,
    )
}

fn build_bounds(tile_pos: TilePos, tile_width: u8, tile_height: u8) -> Rect {
    Rect::new(
        (tile_pos.col * tile_width).into(),
        (tile_pos.row * tile_height).into(),
        tile_width.into(),
        tile_width.into(),
    )
}