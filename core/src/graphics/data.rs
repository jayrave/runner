use crate::rect::Rect;

const NUMBER_TILE_WIDTH: u32 = 82;
const NUMBER_TILE_HEIGHT: u32 = 114;
const PLATFORM_TILE_DIMENSION: u8 = 64;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum TileSheet {
    Cloud,
    Character,
    Enemy,
    Icon,
    Letter,
    Number,
    Platform,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Tile {
    Cloud { tile: CloudTile },
    Character { tile: CharacterTile },
    Enemy { tile: EnemyTile },
    Number { tile: NumberTile },
    Platform { tile: PlatformTile },
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum CloudTile {
    Cloud1,
    Cloud2,
    Cloud3,
    Cloud4,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum CharacterTile {
    Hit,
    Jump,
    Slide,
    Still,
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
pub enum IconTile {
    Up,
    Down,
    Left,
    Right,
    Retry,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum LetterTile {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum NumberTile {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
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

pub fn build_tile_data(tile: Tile) -> TileData {
    let tile_sheet: TileSheet;
    let bounds_in_tile_sheet: Rect;

    match tile {
        Tile::Cloud { tile } => {
            tile_sheet = TileSheet::Cloud;
            bounds_in_tile_sheet = match tile {
                CloudTile::Cloud1 => Rect::new(250, 365, 203, 121),
                CloudTile::Cloud2 => Rect::new(281, 0, 196, 156),
                CloudTile::Cloud3 => Rect::new(0, 865, 216, 139),
                CloudTile::Cloud4 => Rect::new(0, 363, 250, 146),
            }
        }

        Tile::Character { tile } => {
            tile_sheet = TileSheet::Character;
            bounds_in_tile_sheet = match tile {
                CharacterTile::Hit => Rect::new(385, 34, 95, 90),
                CharacterTile::Jump => Rect::new(105, 28, 80, 99),
                CharacterTile::Slide => Rect::new(102, 182, 87, 74),
                CharacterTile::Still => Rect::new(16, 32, 64, 96),
                CharacterTile::Run1 => Rect::new(587, 293, 73, 88),
                CharacterTile::Run2 => Rect::new(678, 287, 83, 94),
                CharacterTile::Run3 => Rect::new(768, 285, 96, 91),
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

        Tile::Number { tile } => {
            tile_sheet = TileSheet::Number;
            bounds_in_tile_sheet = match tile {
                NumberTile::Zero => build_number_bounds(84, 116),
                NumberTile::One => build_number_bounds(252, 116),
                NumberTile::Two => build_number_bounds(0, 116),
                NumberTile::Three => build_number_bounds(420, 0),
                NumberTile::Four => build_number_bounds(168, 116),
                NumberTile::Five => build_number_bounds(252, 0),
                NumberTile::Six => build_number_bounds(168, 0),
                NumberTile::Seven => build_number_bounds(0, 0),
                NumberTile::Eight => build_number_bounds(336, 0),
                NumberTile::Nine => build_number_bounds(84, 0),
            }
        }

        Tile::Platform { tile } => {
            tile_sheet = TileSheet::Platform;
            bounds_in_tile_sheet = match tile {
                PlatformTile::GrassyGround => build_platform_bounds(0, 0),
                PlatformTile::Ground => build_platform_bounds(0, 3),
            }
        }
    };

    TileData {
        tile,
        tile_sheet,
        bounds_in_tile_sheet,
    }
}

fn build_number_bounds(x: i32, y: i32) -> Rect {
    Rect::new(x, y, NUMBER_TILE_WIDTH, NUMBER_TILE_HEIGHT)
}

fn build_platform_bounds(row: u8, col: u8) -> Rect {
    Rect::new(
        i32::from(col) * PLATFORM_TILE_DIMENSION as i32,
        i32::from(row) * PLATFORM_TILE_DIMENSION as i32,
        PLATFORM_TILE_DIMENSION.into(),
        PLATFORM_TILE_DIMENSION.into(),
    )
}
