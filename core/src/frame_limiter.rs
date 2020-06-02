use std::thread::sleep;
use std::time::Duration;
use instant::Instant;

/// Same as the `sleep` strategy from Amethyst
pub struct FrameLimiter {
    frame_duration: Duration,
    last_call: Instant,
}

impl FrameLimiter {
    pub fn new(fps: u32) -> Self {
        Self {
            frame_duration: Duration::from_secs(1) / fps,
            last_call: Instant::now(),
        }
    }

    pub fn limit_as_required(&mut self) {
        loop {
            let elapsed = Instant::now() - self.last_call;
            if elapsed >= self.frame_duration {
                break;
            } else {
                sleep(self.frame_duration - elapsed);
            }
        }

        self.last_call = Instant::now();
    }
}
