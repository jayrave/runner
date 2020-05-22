pub struct GamePlay {
    is_over: bool,
}

impl GamePlay {
    pub fn new() -> GamePlay {
        GamePlay { is_over: false }
    }

    pub fn mark_over(&mut self) {
        self.is_over = true
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }
}
