use crate::components::Drawable;
use crate::data::WorldData;
use crate::graphics::data::TileSheet;
use crate::graphics::textures;
use crate::resources::GamePlayTick;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use specs::join::Join;
use specs::ReadStorage;
use std::convert::TryFrom;

// Tiles should be drawn in a particular order to get the correct Z-index cheaply.
// This should be changed if the tiles drawn are going to change
const TILE_DRAW_ORDER: [TileSheet; 3] =
    [TileSheet::Platform, TileSheet::Enemy, TileSheet::Character];

pub struct Renderer<'a> {
    world_data: WorldData,
    canvas: WindowCanvas,
    textures: textures::Textures<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(
        world_data: WorldData,
        canvas: WindowCanvas,
        textures: textures::Textures<'a>,
    ) -> Renderer {
        Renderer {
            world_data,
            canvas,
            textures,
        }
    }

    pub fn draw_if_required(
        &mut self,
        game_play_tick: &GamePlayTick,
        drawables: Option<ReadStorage<Drawable>>,
    ) {
        // There is no use in drawing if the game didn't move forward
        if let Some(drawables_storage) = drawables {
            if game_play_tick.ticked() {
                self.draw(drawables_storage);
            }
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
                        TileSheet::Platform => &self.textures.platform_texture,
                    };

                    self.canvas
                        .copy(
                            texture,
                            drawable.tile_data.bounds_in_tile_sheet,
                            Renderer::world_to_screen_coordinates(
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
