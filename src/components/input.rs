use specs::Component;
use specs::VecStorage;

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct InputControlled {
    input: Option<data::Input>,
}

impl InputControlled {
    pub fn new() -> InputControlled {
        InputControlled { input: None }
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
