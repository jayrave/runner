use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct Textures<'a> {
    pub cloud_texture: Texture<'a>,
    pub character_texture: Texture<'a>,
    pub enemy_texture: Texture<'a>,
    pub platform_texture: Texture<'a>,
    pub number_texture: Texture<'a>,
}

impl<'a> Textures<'a> {
    pub fn load_from_files(texture_creator: &'a TextureCreator<WindowContext>) -> Textures<'a> {
        // TODO - Couldn't find a way to hold `creator` in the same struct too!
        // Compiler kept complaining & none of my workarounds did anything :(
        let character_texture =
            Textures::load_from_file("character_tile_sheet.png", texture_creator);

        let cloud_texture = Textures::load_from_file("cloud_tile_sheet.png", texture_creator);
        let enemy_texture = Textures::load_from_file("enemy_tile_sheet.png", texture_creator);
        let platform_texture = Textures::load_from_file("platform_tile_sheet.png", texture_creator);
        let number_texture = Textures::load_from_file("number_tile_sheet.png", texture_creator);

        Textures {
            cloud_texture,
            character_texture,
            enemy_texture,
            platform_texture,
            number_texture,
        }
    }

    fn load_from_file(
        filename: &str,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Texture<'a> {
        // Instead of a common top-level dir, we store our assets in quicksilver's static
        // library because the current way of building a WASM module mandates that
        texture_creator
            .load_texture(format!("../quicksilver_frontend/static/{}", filename))
            .unwrap_or_else(|_| panic!("Couldn't load texture: {}", filename))
    }
}
