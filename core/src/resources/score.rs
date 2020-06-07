pub struct Score {
    score: u64,
}

impl Score {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    pub fn update(&mut self, score: u64) {
        self.score = score
    }
}
