use rust_embed::RustEmbed;
use sdl2::image::ImageRWops;
use sdl2::render::{Texture, TextureCreator};
use sdl2::rwops::RWops;
use sdl2::video::WindowContext;

#[derive(RustEmbed)]
#[folder = "../assets_processed/"]
struct Asset;

pub struct Textures<'a> {
    pub cloud_texture: Texture<'a>,
    pub character_texture: Texture<'a>,
    pub enemy_texture: Texture<'a>,
    pub icon_texture: Texture<'a>,
    pub letter_texture: Texture<'a>,
    pub number_texture: Texture<'a>,
    pub platform_texture: Texture<'a>,
}

impl<'a> Textures<'a> {
    pub fn load_from_files(texture_creator: &'a TextureCreator<WindowContext>) -> Textures<'a> {
        // TODO - Couldn't find a way to hold `creator` in the same struct too!
        // Compiler kept complaining & none of my workarounds did anything :(
        let character_texture =
            Textures::load_from_file("character_tile_sheet.png", texture_creator);

        let cloud_texture = Textures::load_from_file("cloud_tile_sheet.png", texture_creator);
        let enemy_texture = Textures::load_from_file("enemy_tile_sheet.png", texture_creator);
        let icon_texture = Textures::load_from_file("icon_tile_sheet.png", texture_creator);
        let letter_texture = Textures::load_from_file("letter_tile_sheet.png", texture_creator);
        let platform_texture = Textures::load_from_file("platform_tile_sheet.png", texture_creator);
        let number_texture = Textures::load_from_file("number_tile_sheet.png", texture_creator);

        Textures {
            cloud_texture,
            character_texture,
            enemy_texture,
            icon_texture,
            letter_texture,
            platform_texture,
            number_texture,
        }
    }

    fn load_from_file(
        filename: &str,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Texture<'a> {
        let bytes = Asset::get(filename).unwrap();
        let rw_ops = RWops::from_bytes(&bytes)
            .unwrap_or_else(|_| panic!("Couldn't load rwops: {}", filename));

        let surface = rw_ops
            .load_png()
            .unwrap_or_else(|_| panic!("Couldn't load surface: {}", filename));

        texture_creator
            .create_texture_from_surface(surface)
            .unwrap_or_else(|_| panic!("Could't load texture: {}", filename))
    }
}
