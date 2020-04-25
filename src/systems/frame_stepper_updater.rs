use crate::resources::FrameStepper;
use specs::System;
use specs::WriteExpect;

pub struct FrameStepperUpdater;

impl<'a> System<'a> for FrameStepperUpdater {
    type SystemData = WriteExpect<'a, FrameStepper>;

    fn run(&mut self, mut frame_stepper: Self::SystemData) {
        frame_stepper.update();
    }
}
