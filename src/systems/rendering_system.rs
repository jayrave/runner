use crate::components::Drawable;
use crate::WorldData;
use crate::graphics::data::TileSheet;
use crate::graphics::textures;
use crate::resources::FrameStepper;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use specs::join::Join;
use specs::{ReadExpect, ReadStorage, System};

pub struct RenderingSystem<'a> {
    world_data: WorldData,
    canvas: WindowCanvas,
    textures: textures::Textures<'a>,
}

impl<'a> RenderingSystem<'a> {
    pub fn new(world_data: WorldData, canvas: WindowCanvas, textures: textures::Textures<'a>) -> RenderingSystem {
        RenderingSystem { world_data, canvas, textures }
    }

    fn world_coords_to_screen_coords(&self, tile_world_bounds: &Rect) -> Rect {
        let half_world_width = self.world_data.world_width() / 2;
        let half_world_height = self.world_data.world_height() / 2;
        Rect::new(
            tile_world_bounds.x() + half_world_width as i32,
            tile_world_bounds.y() + half_world_height as i32,
            tile_world_bounds.width(),
            tile_world_bounds.height(),
        )
    }

    fn draw(&mut self, data: ReadStorage<Drawable>) {
        self.canvas.set_draw_color(self.world_data.sky_color());
        self.canvas.clear();

        for drawable in data.join() {
            let texture = match drawable.tile_data.tile_sheet {
                TileSheet::Character => &self.textures.character_texture,
                TileSheet::Environment => &self.textures.environment_texture,
            };

            self.canvas
                .copy(
                    texture,
                    drawable.tile_data.bounds_in_tile_sheet,
                    self.world_coords_to_screen_coords(&drawable.world_bounds),
                )
                .expect("Couldn't draw texture");
        }

        self.canvas.present();
    }
}

impl<'a, 'b> System<'a> for RenderingSystem<'b> {
    type SystemData = (ReadStorage<'a, Drawable>, ReadExpect<'a, FrameStepper>);

    fn run(&mut self, data: Self::SystemData) {
        if data.1.should_update_frame_buffer() {
            self.draw(data.0);
        }
    }
}
