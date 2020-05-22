pub struct GamePlay {
    is_started: bool,
    is_over: bool,
}

impl GamePlay {
    pub fn new() -> GamePlay {
        GamePlay {
            is_started: false,
            is_over: false,
        }
    }

    pub fn mark_started(&mut self) {
        self.is_started = true
    }

    pub fn mark_over(&mut self) {
        self.is_over = true
    }

    pub fn is_started(&self) -> bool {
        self.is_started
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }

    pub fn should_allow(&self) -> bool {
        self.is_started && !self.is_over
    }
}
