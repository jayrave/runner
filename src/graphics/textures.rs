use sdl2::image::LoadTexture;
use sdl2::render::WindowCanvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct Textures<'a> {
    character_texture: Texture<'a>,
    environment_texture: Texture<'a>,
}

impl Textures {
    pub fn load_from_files(canvas: &WindowCanvas) -> Textures {
        let texture_creator = canvas.texture_creator();
        Textures {
            character_texture: Textures::load_from_file(
                "character_tile_sheet.png",
                &texture_creator,
            ),
            environment_texture: Textures::load_from_file(
                "environment_tile_sheet.png",
                &texture_creator,
            ),
        }
    }

    fn load_from_file(filename: &str, texture_creator: &TextureCreator<WindowContext>) -> Texture {
        texture_creator
            .load_texture(format!("assets/{}", filename))
            .expect(&format!("Couldn't load texture: {}", filename))
    }
}
