use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

pub const GAME_NAME: &str = "Runner";
pub const SKY_COLOR: Color = Color::RGB(58, 154, 216);

pub const WORLD_LEFT: i32 = -400;
pub const WORLD_RIGHT: i32 = 400;
pub const WORLD_TOP: i32 = -300;
pub const WORLD_BOTTOM: i32 = 300;
pub const WORLD_WIDTH: u32 = 800;
pub const WORLD_HEIGHT: u32 = 600;
