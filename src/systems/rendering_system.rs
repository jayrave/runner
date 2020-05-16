use crate::components::Drawable;
use crate::data::WorldData;
use crate::graphics::data::TileSheet;
use crate::graphics::textures;
use crate::resources::GameTick;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use specs::join::Join;
use specs::shred::ResourceId;
use specs::SystemData;
use specs::World;
use specs::{ReadExpect, ReadStorage, System};
use std::convert::TryFrom;

// Tiles should be drawn in a particular order to get the correct Z-index cheaply.
// This should be changed if the tiles drawn are going to change
const TILE_DRAW_ORDER: [TileSheet; 3] = [
    TileSheet::Environment,
    TileSheet::Enemy,
    TileSheet::Character,
];

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
        let mut screen_coordinates = *tile_world_bounds;
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
        for tile_sheet in TILE_DRAW_ORDER.iter() {
            for drawable in drawables_storage.join() {
                if drawable.tile_data.tile_sheet == *tile_sheet {
                    let texture = match drawable.tile_data.tile_sheet {
                        TileSheet::Character => &self.textures.character_texture,
                        TileSheet::Enemy => &self.textures.enemy_texture,
                        TileSheet::Environment => &self.textures.environment_texture,
                    };

                    self.canvas
                        .copy(
                            texture,
                            drawable.tile_data.bounds_in_tile_sheet,
                            RenderingSystem::world_to_screen_coordinates(
                                &drawable.world_bounds,
                                &viewport,
                            ),
                        )
                        .expect("Couldn't draw texture");
                }
            }
        }

        self.canvas.present();
    }
}

#[derive(SystemData)]
pub struct RenderingSystemData<'a> {
    drawables_storage: ReadStorage<'a, Drawable>,
    game_tick: ReadExpect<'a, GameTick>,
}

impl<'a, 'b> System<'a> for RenderingSystem<'b> {
    type SystemData = RenderingSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        // There is no use in drawing a frame if the systems weren't
        // even asked to update their animation
        if data.game_tick.ticked() {
            self.draw(data.drawables_storage);
        }
    }
}
