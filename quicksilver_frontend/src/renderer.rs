use crate::color;
use crate::images::Images;
use quicksilver::geom::{Rectangle, Vector};
use quicksilver::{Graphics, Window};
use runner_core::components::Drawable;
use runner_core::data::WorldData;
use runner_core::graphics::data::TileSheet;
use runner_core::rect::Rect;
use specs::join::Join;
use specs::ReadStorage;

// Tiles should be drawn in a particular order to get the correct Z-index cheaply.
// This should be changed if the tiles drawn are going to change
const TILE_DRAW_ORDER: [TileSheet; 5] = [
    TileSheet::Cloud,
    TileSheet::Platform,
    TileSheet::Enemy,
    TileSheet::Character,
    TileSheet::Number,
];

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
        self.graphics
            .clear(color::qs_color_from(self.world_data.sky_color()));

        // Instead of `self.window.size();`, we are hard setting this to values
        // from world-data since quicksilver is giving different widths for
        // desktop & web targets even though the scale stays the same. Also
        // the canvas element has the expected dimension. So, I am not sure what's
        // really happening here!
        let viewport = Vector::new(
            self.world_data.bounds().width(),
            self.world_data.bounds().height(),
        );

        for tile_sheet in TILE_DRAW_ORDER.iter() {
            for drawable in drawables_storage.join() {
                if drawable.tile_data.tile_sheet == *tile_sheet {
                    let image = match drawable.tile_data.tile_sheet {
                        TileSheet::Cloud => &self.images.cloud_texture,
                        TileSheet::Character => &self.images.character_texture,
                        TileSheet::Enemy => &self.images.enemy_texture,
                        TileSheet::Number => &self.images.number_texture,
                        TileSheet::Platform => &self.images.platform_texture,
                    };

                    self.graphics.draw_subimage(
                        &image,
                        Renderer::qs_rect_from(drawable.tile_data.bounds_in_tile_sheet),
                        Renderer::world_to_screen_coordinates(&drawable.world_bounds, viewport),
                    );
                }
            }
        }

        self.graphics
            .present(&self.window)
            .expect("Not able to present the window");
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
