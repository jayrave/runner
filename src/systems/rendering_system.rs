use crate::constants;
use crate::graphics::textures;
use sdl2::render::WindowCanvas;
use specs::System;

pub struct RenderingSystem<'a> {
    canvas: WindowCanvas,
    textures: textures::Textures<'a>,
}

impl<'a> RenderingSystem<'a> {
    pub fn new(canvas: WindowCanvas, textures: textures::Textures<'a>) -> RenderingSystem {
        RenderingSystem { canvas, textures }
    }
}

impl<'a, 'b> System<'a> for RenderingSystem<'b> {
    type SystemData = ();

    fn run(&mut self, _data: Self::SystemData) {
        self.canvas.set_draw_color(constants::SKY_COLOR);
        self.canvas.clear();
        self.canvas.present();
    }
}
