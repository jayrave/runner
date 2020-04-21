use specs::System;
use sdl2::render::WindowCanvas;
use crate::constants;

pub struct RenderingSystem {
    canvas: WindowCanvas
}

impl RenderingSystem {
    pub fn new(canvas: WindowCanvas) -> RenderingSystem {
        RenderingSystem {
            canvas
        }
    }
}

impl<'a> System<'a> for RenderingSystem {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {
        self.canvas.set_draw_color(constants::SKY_COLOR);
        self.canvas.clear();
        self.canvas.present();
    }
}
