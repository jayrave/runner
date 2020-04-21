use specs::System;
use std::time;
use std::time::Duration;
use std::time::SystemTime;

const NANO_SECONDS_IN_A_FRAME: u64 = 1_000_000_000 / 60;

pub struct PowerOptimizerSystem {
    last_frame_at: SystemTime
}

impl PowerOptimizerSystem {
    pub fn new() -> PowerOptimizerSystem {
        PowerOptimizerSystem {
            last_frame_at: SystemTime::now()
        }
    }
}

impl<'a> System<'a> for PowerOptimizerSystem {
    type SystemData = ();

    fn run(&mut self, _data: Self::SystemData) {
        let nano_seconds_elapsed = self.last_frame_at.elapsed().unwrap().as_nanos();
        self.last_frame_at = SystemTime::now();

        // TODO - how to prevent all the casting here?
        // If I do the same casting with unsigned integers, it leads to
        // "attempt to subtract with overflow"
        let nano_seconds_to_sleep = NANO_SECONDS_IN_A_FRAME as i128 - nano_seconds_elapsed as i128;
        if nano_seconds_to_sleep > 0 {
            std::thread::sleep(Duration::from_nanos(nano_seconds_to_sleep as u64));
        }
    }
}
