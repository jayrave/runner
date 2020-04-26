use sdl2::pixels::Color;

use std::convert::TryFrom;

#[derive(Copy, Clone, Debug)]
pub struct WorldData {
    sky_color: Color,
    world_width: u32,
    world_height: u32,
    ground_speed_in_wc: u8,
}

impl WorldData {
    pub fn new() -> WorldData {
        WorldData {
            sky_color: Color::RGB(58, 154, 216),
            world_width: 800,
            world_height: 600,
            ground_speed_in_wc: 2
        }
    }

    pub fn sky_color(&self) -> Color {
        self.sky_color
    }

    pub fn world_width(&self) -> u32 {
        self.world_width
    }

    pub fn world_height(&self) -> u32 {
        self.world_height
    }

    pub fn world_left(&self) -> i32 {
        -self.world_right()
    }

    pub fn world_top(&self) -> i32 {
        -self.world_bottom()
    }

    pub fn world_right(&self) -> i32 {
        i32::try_from(self.world_width / 2).expect("World width to right failed")
    }

    pub fn world_bottom(&self) -> i32 {
        i32::try_from(self.world_height / 2).expect("World height to bottom failed")
    }

    pub fn world_surface_at(&self) -> i32 {
        self.world_bottom() / 2
    }

    pub fn ground_speed_in_wc(&self) -> u8 {
        self.ground_speed_in_wc
    }
}
