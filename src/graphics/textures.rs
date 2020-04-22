use sdl2::image::LoadTexture;
use sdl2::render::WindowCanvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct Textures<'a> {
    character_texture: Texture<'a>,
    environment_texture: Texture<'a>,
}

impl<'a> Textures<'a> {
    pub fn load_from_files(texture_creator: &'a TextureCreator<WindowContext>) -> Textures<'a> {
        // TODO - Couldn't find a way to hold `creator` in the same struct too!
        // Compiler kept complaining & no one my workarounds did anything :(
        let character_texture =
            Textures::load_from_file("character_tile_sheet.png", texture_creator);

        let environment_texture =
            Textures::load_from_file("environment_tile_sheet.png", texture_creator);

        Textures {
            character_texture,
            environment_texture,
        }
    }

    fn load_from_file(
        filename: &str,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Texture<'a> {
        texture_creator
            .load_texture(format!("assets/{}", filename))
            .expect(&format!("Couldn't load texture: {}", filename))
    }
}
