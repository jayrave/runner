use std::time::Duration;
use std::time::SystemTime;

/// Changing this number could affect how the simulation is run. For
/// eg., we could be moving a character 5 world coordinates per tick.
/// If tick timing is changed, all the related computations may have
/// to be updated accordingly too
pub const MILLISECONDS_IN_A_TICK: u8 = 16;

/// To track data about the ticks we have dealt or should deal with.
///
/// What is a tick? => this is how granular our systems can get. To
/// provide hardware/frame rate independent experience, we need to
/// run our systems once to cover a set amount of time (one tick)
/// no matter which system we are run on
pub struct GamePlay {
    /// Flags to control whether game play can take place or not
    is_started: bool,
    is_over: bool,

    /// Number of ticks animated by our systems since the start of
    /// of the game
    ticks_animated: u64,

    /// Number of ticks to animate by our systems before the next
    /// `update` call to this.
    ///
    /// Note: we don't enforce this in any way. This is a contract
    /// that has to held
    ticks_to_animate: u64,
    last_ticks_to_animate_update_at: SystemTime,
}

impl GamePlay {
    pub fn new() -> GamePlay {
        GamePlay {
            is_started: false,
            is_over: false,
            ticks_animated: 0,
            ticks_to_animate: 0,

            // This will be overwritten when the play actually starts
            last_ticks_to_animate_update_at: SystemTime::now(),
        }
    }

    pub fn is_started(&self) -> bool {
        self.is_started
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }

    pub fn is_allowed(&self) -> bool {
        self.is_started && !self.is_over
    }

    pub fn ticks_animated(&self) -> u64 {
        self.ticks_animated
    }

    pub fn ticks_to_animate(&self) -> u64 {
        self.ticks_to_animate
    }

    pub fn ticked(&self) -> bool {
        self.ticks_to_animate > 0
    }

    pub fn mark_started(&mut self) {
        self.is_started = true;

        // The play is only marked now as started. Start computing
        // the ticks relative to this time. Tried making this prop
        // an `Option` but that didn't make the code any easier!
        self.last_ticks_to_animate_update_at = SystemTime::now();
    }

    pub fn mark_over(&mut self) {
        self.is_over = true
    }

    /// `ticks_to_animate` will be reset for every call to `update_if_allowed`
    /// if game play is allowed. Make sure that systems have stepped through
    /// these many ticks. Results are undefined if this isn't done
    pub fn update_if_allowed(&mut self) {
        if self.is_allowed() {
            // Make sure to update `animated` count & reset `to_animate` count
            self.ticks_animated += self.ticks_to_animate;
            self.ticks_to_animate = 0;

            // We want to make sure we advance our physics in deterministic steps
            // all the time to be hardware independent
            let mut ms_elapsed = self
                .last_ticks_to_animate_update_at
                .elapsed()
                .unwrap()
                .as_millis();

            let mut ticks_to_animate = 0u64;
            while ms_elapsed > MILLISECONDS_IN_A_TICK.into() {
                ticks_to_animate += 1;
                ms_elapsed -= u128::from(MILLISECONDS_IN_A_TICK)
            }

            if ticks_to_animate > 0 {
                self.ticks_to_animate = ticks_to_animate;
                self.last_ticks_to_animate_update_at +=
                    Duration::from_millis(ticks_to_animate * u64::from(MILLISECONDS_IN_A_TICK));
            }
        }
    }
}
