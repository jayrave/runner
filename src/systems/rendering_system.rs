use crate::constants;
use sdl2::render::WindowCanvas;
use specs::System;

pub struct RenderingSystem {
    canvas: WindowCanvas,
}

impl RenderingSystem {
    pub fn new(canvas: WindowCanvas) -> RenderingSystem {
        RenderingSystem { canvas }
    }
}

impl<'a> System<'a> for RenderingSystem {
    type SystemData = ();

    fn run(&mut self, _data: Self::SystemData) {
        self.canvas.set_draw_color(constants::SKY_COLOR);
        self.canvas.clear();
        self.canvas.present();
    }
}
