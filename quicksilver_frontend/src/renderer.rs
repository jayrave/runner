use crate::color;
use crate::images::Images;
use quicksilver::geom::{Rectangle, Vector};
use quicksilver::{Graphics, Window};
use runner_core::components::Drawable;
use runner_core::data::WorldData;
use runner_core::graphics::data::TileSheet;
use runner_core::rect::Rect;
use runner_core::render;
use specs::join::Join;
use specs::ReadStorage;

pub struct Renderer {
    world_data: WorldData,
    window: Window,
    graphics: Graphics,
    images: Images,
}

impl Renderer {
    pub fn new(
        world_data: WorldData,
        window: Window,
        graphics: Graphics,
        images: Images,
    ) -> Renderer {
        Renderer {
            world_data,
            window,
            graphics,
            images,
        }
    }

    pub fn draw(&mut self, drawables_storage: ReadStorage<Drawable>) {
        // No need for alpha compositing for game play drawables
        let sky_color = self.world_data.sky_color();
        self.graphics.clear(color::qs_color_from(sky_color));

        // Display game play
        self.draw_drawables(
            &drawables_storage,
            &render::GAME_PLAY_DRAW_ORDER,
            |_graphics: &mut Graphics| {},
        );

        // For the next part to show instructions, we are going to put up a different
        // color rect for being really obvious
        let viewport = self.viewport();
        let bg_color = self.world_data.instructions_bg_color();
        let world_bounds = self.world_data.bounds();
        self.draw_drawables(
            &drawables_storage,
            &render::INSTRUCTIONS_DRAW_ORDER,
            |graphics: &mut Graphics| {
                // This is where the one time setup for the bg color happens
                graphics.fill_rect(
                    &Renderer::world_to_screen_coordinates(&world_bounds, viewport),
                    color::qs_color_from(bg_color),
                );
            },
        );

        self.graphics
            .present(&self.window)
            .expect("Not able to present the window");
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
        F: Fn(&mut Graphics) -> (),
    {
        let viewport = self.viewport();
        let mut one_time_setup_done = false;

        for tile_sheet in tile_sheet_draw_order.iter() {
            for drawable in drawables_storage.join() {
                if drawable.tile_data.tile_sheet == *tile_sheet {
                    if !one_time_setup_done {
                        one_time_setup(&mut self.graphics);
                        one_time_setup_done = true
                    }

                    let image = match drawable.tile_data.tile_sheet {
                        TileSheet::Cloud => &self.images.cloud_texture,
                        TileSheet::Character => &self.images.character_texture,
                        TileSheet::Enemy => &self.images.enemy_texture,
                        TileSheet::Icon => &self.images.icon_texture,
                        TileSheet::Letter => &self.images.letter_texture,
                        TileSheet::Platform => &self.images.platform_texture,
                        TileSheet::Number => &self.images.number_texture,
                    };

                    self.graphics.draw_subimage(
                        &image,
                        Renderer::qs_rect_from(drawable.tile_data.bounds_in_tile_sheet),
                        Renderer::world_to_screen_coordinates(&drawable.world_bounds, viewport),
                    );
                }
            }
        }
    }

    fn viewport(&self) -> Vector {
        // Instead of `self.window.size();`, we are hard setting this to values
        // from world-data since quicksilver is giving different widths for
        // desktop & web targets even though the scale stays the same. Also
        // the canvas element has the expected dimension. So, I am not sure what's
        // really happening here!
        Vector::new(
            self.world_data.bounds().width(),
            self.world_data.bounds().height(),
        )
    }

    fn world_to_screen_coordinates(tile_world_bounds: &Rect, viewport: Vector) -> Rectangle {
        let mut screen_coordinates = *tile_world_bounds;
        screen_coordinates.offset((viewport.x / 2.0) as i32, (viewport.y / 2.0) as i32);

        Renderer::qs_rect_from(screen_coordinates)
    }

    /// Instead of `Into` doing it this way since both the trait
    /// & the struct are from outside this crate & so Rust wouldn't
    /// allow this
    fn qs_rect_from(rect: Rect) -> Rectangle {
        Rectangle::new(
            Vector::new(rect.x() as f32, rect.y() as f32),
            Vector::new(rect.width() as f32, rect.height() as f32),
        )
    }
}
