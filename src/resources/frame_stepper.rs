use sdl2::hint::set;
use std::time::Duration;
use std::time::SystemTime;

const MILLISECONDS_IN_A_FRAME: u8 = 16;

pub struct FrameStepper {
    last_update_at: SystemTime,
    frame_count_animated: u64,
    frame_count_to_animate: u64,
}

impl FrameStepper {
    pub fn new() -> FrameStepper {
        FrameStepper {
            last_update_at: SystemTime::now(),
            frame_count_animated: 0,
            frame_count_to_animate: 0,
        }
    }

    pub fn frame_count_animated(&self) -> u64 {
        self.frame_count_animated
    }

    pub fn frame_count_to_animate(&self) -> u64 {
        self.frame_count_to_animate
    }

    pub fn should_update_frame_buffer(&self) -> bool {
        self.frame_count_to_animate > 0
    }

    // `frame_count_to_animate` will be reset for every call to `updated`
    pub fn update(&mut self) {
        // Make sure to update `animated` count & reset `to_animate` count
        self.frame_count_animated += self.frame_count_to_animate;
        self.frame_count_to_animate = 0;

        // We want to make sure we advance our physics in deterministic steps
        // all the time to be hardware independent
        let mut ms_elapsed = self.last_update_at.elapsed().unwrap().as_millis();
        let mut frame_count_to_animate = 0;
        while ms_elapsed > MILLISECONDS_IN_A_FRAME.into() {
            frame_count_to_animate += 1;
            ms_elapsed -= MILLISECONDS_IN_A_FRAME as u128
        }

        if frame_count_to_animate > 0 {
            self.frame_count_to_animate = frame_count_to_animate;
            self.last_update_at += Duration::from_millis(
                frame_count_to_animate as u64 * MILLISECONDS_IN_A_FRAME as u64,
            );
        }
    }
}
