use sdl2::hint::set;
use std::time::Duration;
use std::time::SystemTime;

/// 60 FPS
const MILLISECONDS_IN_A_FRAME: u8 = 16;

/// To track data about the frames we have dealt or
/// should deal with
pub struct FrameStepper {
    /// Number of frames animated by our systems since the start of
    /// of the game
    frame_count_animated: u64,

    /// Number of frames to animate by our systems before the next
    /// `update` call to this.
    ///
    /// Note: we don't enforce this in any way. This is a contract
    /// that has to held
    frame_count_to_animate: u64,
    last_animate_count_update_at: SystemTime,
}

impl FrameStepper {
    pub fn new() -> FrameStepper {
        FrameStepper {
            frame_count_animated: 0,
            frame_count_to_animate: 0,
            last_animate_count_update_at: SystemTime::now(),
        }
    }

    pub fn frame_count_animated(&self) -> u64 {
        self.frame_count_animated
    }

    pub fn frame_count_to_animate(&self) -> u64 {
        self.frame_count_to_animate
    }

    pub fn should_update_frame_buffer(&self) -> bool {
        // There is no use in drawing a frame if the systems weren't
        // even asked to update their animation
        self.frame_count_to_animate > 0
    }

    /// `frame_count_to_animate` will be reset for every call to `update`.
    /// Make sure that systems have stepped through these many animation
    /// frames. Results are undefined if this isn't done
    pub fn update(&mut self) {
        // Make sure to update `animated` count & reset `to_animate` count
        self.frame_count_animated += self.frame_count_to_animate;
        self.frame_count_to_animate = 0;

        // We want to make sure we advance our physics in deterministic steps
        // all the time to be hardware independent
        let mut ms_elapsed = self
            .last_animate_count_update_at
            .elapsed()
            .unwrap()
            .as_millis();

        let mut frame_count_to_animate = 0u64;
        while ms_elapsed > MILLISECONDS_IN_A_FRAME.into() {
            frame_count_to_animate += 1;
            ms_elapsed -= u128::from(MILLISECONDS_IN_A_FRAME)
        }

        if frame_count_to_animate > 0 {
            self.frame_count_to_animate = frame_count_to_animate;
            self.last_animate_count_update_at +=
                Duration::from_millis(frame_count_to_animate * u64::from(MILLISECONDS_IN_A_FRAME));
        }
    }
}
