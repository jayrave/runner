use quicksilver::graphics::Image;
use quicksilver::Graphics;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../assets_processed/"]
struct Asset;

pub struct Images {
    pub cloud_texture: Image,
    pub character_texture: Image,
    pub enemy_texture: Image,
    pub icon_texture: Image,
    pub letter_texture: Image,
    pub platform_texture: Image,
    pub number_texture: Image,
}

impl Images {
    pub fn load_from_files(graphics: &Graphics) -> Images {
        Images {
            cloud_texture: Images::load_from_file("cloud_tile_sheet.png", graphics),
            character_texture: Images::load_from_file("character_tile_sheet.png", graphics),
            enemy_texture: Images::load_from_file("enemy_tile_sheet.png", graphics),
            icon_texture: Images::load_from_file("icon_tile_sheet.png", graphics),
            letter_texture: Images::load_from_file("letter_tile_sheet.png", graphics),
            platform_texture: Images::load_from_file("platform_tile_sheet.png", graphics),
            number_texture: Images::load_from_file("number_tile_sheet.png", graphics),
        }
    }

    fn load_from_file(filename: &str, graphics: &Graphics) -> Image {
        let bytes = Asset::get(filename).unwrap();
        Image::from_encoded_bytes(&graphics, &bytes).unwrap_or_else(|_| panic!("Couldn't load file: {}", filename))
    }
}
