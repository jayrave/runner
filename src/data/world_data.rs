use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use std::convert::TryFrom;

#[derive(Copy, Clone, Debug)]
pub struct WorldData {
    sky_color: Color,
    world_bounds: Rect,
}

impl Default for WorldData {
    fn default() -> Self {
        WorldData {
            sky_color: Color::RGB(58, 154, 216),
            world_bounds: Rect::from_center(Point::new(0, 0), 800, 600),
        }
    }
}

impl WorldData {
    pub fn new() -> WorldData {
        WorldData::default()
    }

    pub fn sky_color(&self) -> Color {
        self.sky_color
    }

    pub fn bounds(&self) -> Rect {
        self.world_bounds
    }

    pub fn world_surface_at(&self) -> i32 {
        self.bounds().bottom() / 2
    }
}
