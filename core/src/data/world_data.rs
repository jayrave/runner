use crate::color::Color;
use crate::rect::Rect;

#[derive(Copy, Clone, Debug)]
pub struct WorldData {
    sky_color: Color,
    instructions_bg_color: Color,
    world_bounds: Rect,
}

impl Default for WorldData {
    fn default() -> Self {
        let width: u32 = 800;
        let height: u32 = 400;
        WorldData {
            sky_color: Color::from_rgba(58, 154, 216, u8::max_value()),
            instructions_bg_color: Color::from_rgba(0, 0, 0, u8::max_value() / 2),
            world_bounds: Rect::new(-((width / 2) as i32), -((height / 2) as i32), width, height),
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

    pub fn instructions_bg_color(&self) -> Color {
        self.instructions_bg_color
    }

    pub fn bounds(&self) -> Rect {
        self.world_bounds
    }

    pub fn world_surface_at(&self) -> i32 {
        self.bounds().bottom() / 3
    }
}
