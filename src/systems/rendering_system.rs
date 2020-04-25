use crate::components::Drawable;
use crate::graphics::data::TileSheet;
use crate::graphics::textures;
use crate::resources::FrameStepper;
use crate::WorldData;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::SystemData;
use specs::World;
use specs::{ReadExpect, ReadStorage, System};
use std::convert::TryFrom;

pub struct RenderingSystem<'a> {
    world_data: WorldData,
    canvas: WindowCanvas,
    textures: textures::Textures<'a>,
}

impl<'a> RenderingSystem<'a> {
    pub fn new(
        world_data: WorldData,
        canvas: WindowCanvas,
        textures: textures::Textures<'a>,
    ) -> RenderingSystem {
        RenderingSystem {
            world_data,
            canvas,
            textures,
        }
    }

    fn world_to_screen_coordinates(tile_world_bounds: &Rect, viewport: &Rect) -> Rect {
        let mut screen_coordinates = tile_world_bounds.clone();
        screen_coordinates.offset(
            i32::try_from(viewport.width() / 2).expect("u32/2 is not i32!"),
            i32::try_from(viewport.height() / 2).expect("u32/2 is not i32!"),
        );

        screen_coordinates
    }

    fn draw(&mut self, drawables_storage: ReadStorage<Drawable>) {
        self.canvas.set_draw_color(self.world_data.sky_color());
        self.canvas.clear();

        let viewport = self.canvas.viewport();
        for drawable in drawables_storage.join() {
            let texture = match drawable.tile_data.tile_sheet {
                TileSheet::Character => &self.textures.character_texture,
                TileSheet::Environment => &self.textures.environment_texture,
            };

            self.canvas
                .copy(
                    texture,
                    drawable.tile_data.bounds_in_tile_sheet,
                    RenderingSystem::world_to_screen_coordinates(&drawable.world_bounds, &viewport),
                )
                .expect("Couldn't draw texture");
        }

        self.canvas.present();
    }
}

#[derive(SystemData)]
pub struct RenderingSystemData<'a> {
    drawables_storage: ReadStorage<'a, Drawable>,
    frame_stepper: ReadExpect<'a, FrameStepper>,
}

impl<'a, 'b> System<'a> for RenderingSystem<'b> {
    type SystemData = RenderingSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        if data.frame_stepper.should_update_frame_buffer() {
            self.draw(data.drawables_storage);
        }
    }
}
