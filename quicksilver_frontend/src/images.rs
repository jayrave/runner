use quicksilver::graphics::Image;
use quicksilver::Graphics;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../assets_processed/"]
struct Asset;

pub struct Images {
    pub cloud_image: Image,
    pub character_image: Image,
    pub enemy_image: Image,
    pub icon_image: Image,
    pub letter_image: Image,
    pub platform_image: Image,
    pub number_image: Image,
}

impl Images {
    pub fn load_from_files(graphics: &Graphics) -> Images {
        Images {
            cloud_image: Images::load_from_file("cloud_tile_sheet.png", graphics),
            character_image: Images::load_from_file("character_tile_sheet.png", graphics),
            enemy_image: Images::load_from_file("enemy_tile_sheet.png", graphics),
            icon_image: Images::load_from_file("icon_tile_sheet.png", graphics),
            letter_image: Images::load_from_file("letter_tile_sheet.png", graphics),
            platform_image: Images::load_from_file("platform_tile_sheet.png", graphics),
            number_image: Images::load_from_file("number_tile_sheet.png", graphics),
        }
    }

    fn load_from_file(filename: &str, graphics: &Graphics) -> Image {
        let bytes = Asset::get(filename).unwrap();
        Image::from_encoded_bytes(&graphics, &bytes).unwrap_or_else(|_| panic!("Couldn't load file: {}", filename))
    }
}
