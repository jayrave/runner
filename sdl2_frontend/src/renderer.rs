use crate::color;
use crate::textures::Textures;
use runner_core::components::Drawable;
use runner_core::data::WorldData;
use runner_core::graphics::data::TileSheet;
use runner_core::rect::Rect;
use runner_core::render;
use sdl2::rect::Rect as SdlRect;
use sdl2::render::{BlendMode, WindowCanvas};
use specs::join::Join;
use specs::ReadStorage;
use std::convert::TryFrom;

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
        // No need for alpha compositing for game play drawables
        let sky_color = self.world_data.sky_color();
        self.canvas.set_blend_mode(BlendMode::None);
        self.canvas.set_draw_color(color::sdl_color_from(sky_color));
        self.canvas.clear();

        // Display game play
        self.draw_drawables(
            &drawables_storage,
            &render::GAME_PLAY_DRAW_ORDER,
            |_canvas: &mut WindowCanvas| {},
        );

        // For the next part to show instructions, we are going to put up a different
        // color rect for being really obvious
        let bg_color = self.world_data.instructions_bg_color();
        let world_bounds = self.world_data.bounds();
        self.draw_drawables(
            &drawables_storage,
            &render::INSTRUCTIONS_DRAW_ORDER,
            |canvas: &mut WindowCanvas| {
                // This is where the one time setup for the blend mode &
                // bg color happen
                canvas.set_blend_mode(BlendMode::Blend);
                canvas.set_draw_color(color::sdl_color_from(bg_color));
                canvas.fill_rect(Renderer::world_to_screen_coordinates(
                    &world_bounds,
                    &canvas.viewport(),
                )).expect("Not able to complete rect fill!");
            },
        );

        self.canvas.present();
    }

    /// Even though it is called `one_time_setup`, we can still typing it
    /// as `Fn` instead of `FnOnce` since I can't figure out how to use
    /// that inside a loop!
    fn draw_drawables<F>(
        &mut self,
        drawables_storage: &ReadStorage<Drawable>,
        tile_sheet_draw_order: &[TileSheet],
        one_time_setup: F,
    ) where
        F: Fn(&mut WindowCanvas) -> (),
    {
        let viewport = self.canvas.viewport();
        let mut one_time_setup_done = false;

        for tile_sheet in tile_sheet_draw_order.iter() {
            for drawable in drawables_storage.join() {
                if drawable.tile_data.tile_sheet == *tile_sheet {
                    if !one_time_setup_done {
                        one_time_setup(&mut self.canvas);
                        one_time_setup_done = true
                    }

                    let texture = match drawable.tile_data.tile_sheet {
                        TileSheet::Cloud => &self.textures.cloud_texture,
                        TileSheet::Character => &self.textures.character_texture,
                        TileSheet::Enemy => &self.textures.enemy_texture,
                        TileSheet::Icon => &self.textures.icon_texture,
                        TileSheet::Letter => &self.textures.letter_texture,
                        TileSheet::Platform => &self.textures.platform_texture,
                        TileSheet::Number => &self.textures.number_texture,
                    };

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
