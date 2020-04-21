pub struct GameFinisher {
    should_finish: bool,
}

impl GameFinisher {
    pub fn new() -> GameFinisher {
        GameFinisher {
            should_finish: false,
        }
    }

    pub fn finish(&mut self) {
        self.should_finish = true
    }

    pub fn should_finish(&self) -> bool {
        self.should_finish
    }
}