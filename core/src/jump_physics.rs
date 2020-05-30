use crate::components::input::InputControlled;

pub struct JumpPhysics {
    start_at_tick: u64,
    initial_height: i32,
    initial_jump_velocity: f32,
    gravity: f32,
    is_lower_gravity: bool,
}

impl JumpPhysics {
    pub fn from_ground(
        current_tick: u64,
        ticks_in_max_jump: u8,
        max_jump_height_in_wc: u8,
    ) -> JumpPhysics {
        let gravity = JumpPhysics::compute_gravity(ticks_in_max_jump, max_jump_height_in_wc);
        let initial_velocity = JumpPhysics::compute_initial_velocity(ticks_in_max_jump, gravity);
        JumpPhysics {
            start_at_tick: current_tick,
            initial_height: 0,
            initial_jump_velocity: initial_velocity,
            gravity,
            is_lower_gravity: true,
        }
    }

    pub fn compute_height(&self, current_tick: u64) -> i32 {
        let ticks_since_jump_started = (current_tick - self.start_at_tick) as f32;
        (((self.gravity * ticks_since_jump_started.powf(2.0)) / 2.0)
            + (self.initial_jump_velocity * ticks_since_jump_started)) as i32
            + self.initial_height
    }

    pub fn update_gravity_if_required(
        self,
        current_tick: u64,
        current_height: i32,
        input_ctrl: &InputControlled,
    ) -> JumpPhysics {
        if !self.is_lower_gravity {
            self
        } else {
            let ticks_since_jump_started = (current_tick - self.start_at_tick) as f32;
            let current_velocity =
                self.initial_jump_velocity + (self.gravity * ticks_since_jump_started);

            if input_ctrl.up_engaged() && current_velocity > 0.0 {
                self
            } else {
                JumpPhysics {
                    start_at_tick: current_tick,
                    initial_height: current_height,
                    initial_jump_velocity: 0.0,
                    gravity: self.gravity * 2.0,
                    is_lower_gravity: false,
                }
            }
        }
    }

    fn compute_gravity(ticks_in_max_jump: u8, max_jump_height_in_wc: u8) -> f32 {
        // Derived using math from a GDC talk (for smooth parabolic jump):
        //      GDC link: https://www.gdcvault.com/play/1023559/Math-for-Game-Programmers-Building
        //      Video: https://www.youtube.com/watch?v=hG9SzQxaCm8
        //      Slides: http://www.mathforgameprogrammers.com/gdc2016/GDC2016_Pittman_Kyle_BuildingABetterJump.pdf
        let ticks_to_hit_apex: f32 = ticks_in_max_jump as f32 / 2.0;
        (-2.0 * max_jump_height_in_wc as f32) / (ticks_to_hit_apex.powf(2.0))
    }

    fn compute_initial_velocity(ticks_in_max_jump: u8, gravity: f32) -> f32 {
        // Same method as described for gravity
        let ticks_to_hit_apex: f32 = ticks_in_max_jump as f32 / 2.0;
        -gravity * ticks_to_hit_apex
    }
}
