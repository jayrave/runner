use instant::Instant;
use log::info;

pub struct Fps {
    frame_count_in_window: u32,
    window_start: Instant,
}

impl Fps {
    pub fn new() -> Self {
        Fps {
            frame_count_in_window: 0,
            window_start: Instant::now(),
        }
    }

    pub fn track(&mut self) {
        self.frame_count_in_window += 1;
        let now = Instant::now();
        let window_elapsed_in_seconds = (now - self.window_start).as_secs_f32();

        if window_elapsed_in_seconds > 1.0 {
            let fps_in_window = self.frame_count_in_window as f32 / window_elapsed_in_seconds;
            info!("FPS: {}", fps_in_window);

            self.frame_count_in_window = 0;
            self.window_start = now;
        }
    }
}
