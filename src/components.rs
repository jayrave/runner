use crate::graphics;
use sdl2::rect::Rect;
use specs::Component;
use specs::NullStorage;
use specs::VecStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub tile_data: graphics::data::TileData,
    pub world_bounds: Rect,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Ground;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {
    pub input_based_animation_started_at_frame: Option<u64>,
}

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct UserControlled {
    input: Option<data::Input>,
}

impl UserControlled {
    pub fn new() -> UserControlled {
        UserControlled { input: None }
    }

    pub fn update_input_if_required(&mut self, input: data::Input) {
        self.input = Some(input)
    }

    pub fn consume_input(&mut self) -> Option<data::Input> {
        self.input.take()
    }
}

pub mod data {
    #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Input {
        Up,
        Down,
    }
}
