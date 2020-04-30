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

    pub fn up_engaged(&self) -> bool {
        self.up
    }

    pub fn down_engaged(&self) -> bool {
        self.down
    }

    pub fn left_engaged(&self) -> bool {
        self.left
    }

    pub fn right_engaged(&self) -> bool {
        self.right
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
