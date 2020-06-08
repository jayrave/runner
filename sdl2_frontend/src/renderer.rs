use crate::color;
use crate::textures::Textures;
use runner_core::components::Drawable;
use runner_core::data::WorldData;
use runner_core::graphics::data::TileSheet;
use runner_core::rect::Rect;
use sdl2::rect::Rect as SdlRect;
use sdl2::render::WindowCanvas;
use specs::join::Join;
use specs::ReadStorage;
use std::convert::TryFrom;
use runner_core::render;

pub struct Renderer<'a> {
    world_data: WorldData,
    canvas: WindowCanvas,
    textures: Textures<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(world_data: WorldData, canvas: WindowCanvas, textures: Textures<'a>) -> Renderer {
        Renderer {
            world_data,
            canvas,
            textures,
        }
    }

    pub fn draw(&mut self, drawables_storage: ReadStorage<Drawable>) {
        self.canvas
            .set_draw_color(color::sdl_color_from(self.world_data.sky_color()));
        self.canvas.clear();

        let viewport = self.canvas.viewport();
        for tile_sheet in render::GAME_PLAY_DRAW_ORDER.iter() {
            for drawable in drawables_storage.join() {
                if drawable.tile_data.tile_sheet == *tile_sheet {
                    let texture = match drawable.tile_data.tile_sheet {
                        TileSheet::Cloud => Some(&self.textures.cloud_texture),
                        TileSheet::Character => Some(&self.textures.character_texture),
                        TileSheet::Enemy => Some(&self.textures.enemy_texture),
                        TileSheet::Platform => Some(&self.textures.platform_texture),
                        TileSheet::Number => Some(&self.textures.number_texture),
                        _ => None,
                    };

                    if let Some(ref texture) = texture {
                        self.canvas
                            .copy(
                                texture,
                                Renderer::sdl_rect_from(drawable.tile_data.bounds_in_tile_sheet),
                                Renderer::world_to_screen_coordinates(
                                    &drawable.world_bounds,
                                    &viewport,
                                ),
                            )
                            .expect("Couldn't draw texture");
                    }
                }
            }
        }

        self.canvas.present();
    }

    fn world_to_screen_coordinates(tile_world_bounds: &Rect, viewport: &SdlRect) -> SdlRect {
        let mut screen_coordinates = *tile_world_bounds;
        screen_coordinates.offset(
            i32::try_from(viewport.width() / 2).expect("u32/2 is not i32!"),
            i32::try_from(viewport.height() / 2).expect("u32/2 is not i32!"),
        );

        Renderer::sdl_rect_from(screen_coordinates)
    }

    /// Instead of `Into` doing it this way since both the trait
    /// & the struct are from outside this crate & so Rust wouldn't
    /// allow this
    fn sdl_rect_from(rect: Rect) -> SdlRect {
        SdlRect::new(rect.x(), rect.y(), rect.width(), rect.height())
    }
}
