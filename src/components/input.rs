use crate::components::input::data::Direction;
use specs::Component;
use specs::VecStorage;

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct InputControlled {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl InputControlled {
    pub fn new() -> InputControlled {
        InputControlled::default()
    }

    pub fn update_key_down(&mut self, direction: data::Direction) {
        match direction {
            Direction::Up => self.up = true,
            Direction::Down => self.down = true,
            Direction::Left => self.left = true,
            Direction::Right => self.right = true,
        }
    }

    pub fn update_key_up(&mut self, direction: data::Direction) {
        match direction {
            Direction::Up => self.up = false,
            Direction::Down => self.down = false,
            Direction::Left => self.left = false,
            Direction::Right => self.right = false,
        }
    }

    pub fn consume_input(&mut self) -> Option<data::Direction> {
        if self.up {
            Some(data::Direction::Up)
        } else if self.down {
            Some(data::Direction::Down)
        } else if self.left {
            Some(data::Direction::Left)
        } else if self.right {
            Some(data::Direction::Right)
        } else {
            None
        }
    }
}

pub mod data {
    #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
}
