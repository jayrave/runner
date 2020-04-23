use crate::components::Drawable;
use crate::constants;
use crate::graphics::data::TileSheet;
use crate::graphics::textures;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use specs::join::Join;
use specs::{ReadStorage, System};

pub struct RenderingSystem<'a> {
    canvas: WindowCanvas,
    textures: textures::Textures<'a>,
}

impl<'a> RenderingSystem<'a> {
    pub fn new(canvas: WindowCanvas, textures: textures::Textures<'a>) -> RenderingSystem {
        RenderingSystem { canvas, textures }
    }

    fn world_coords_to_screen_coords(tile_world_bounds: &Rect) -> Rect {
        let half_world_width = constants::WORLD_WIDTH / 2;
        let half_world_height = constants::WORLD_HEIGHT / 2;
        Rect::new(
            tile_world_bounds.x() + half_world_width as i32,
            tile_world_bounds.y() + half_world_height as i32,
            tile_world_bounds.width(),
            tile_world_bounds.height(),
        )
    }
}

impl<'a, 'b> System<'a> for RenderingSystem<'b> {
    type SystemData = ReadStorage<'a, Drawable>;

    fn run(&mut self, data: Self::SystemData) {
        self.canvas.set_draw_color(constants::SKY_COLOR);
        self.canvas.clear();

        for drawable in data.join() {
            let texture = match drawable.tile_data.tile_sheet {
                TileSheet::Character => &self.textures.character_texture,
                TileSheet::Environment => &self.textures.environment_texture,
            };

            self.canvas.copy(
                texture,
                drawable.tile_data.bounds_in_tile_sheet,
                RenderingSystem::world_coords_to_screen_coords(&drawable.world_bounds),
            ).expect("Couldn't draw texture");
        }

        self.canvas.present();
    }
}
