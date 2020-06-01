use quicksilver::{
    graphics::{Color, Image},
    Graphics,
};

pub struct Images {
    pub cloud_texture: Image,
    pub character_texture: Image,
    pub enemy_texture: Image,
    pub platform_texture: Image,
}

impl Images {
    pub async fn load_from_files(graphics: &Graphics) -> Images {
        Images {
            cloud_texture: Images::load_from_file("cloud_tile_sheet.png", graphics).await,
            character_texture: Images::load_from_file("character_tile_sheet.png", graphics).await,
            enemy_texture: Images::load_from_file("enemy_tile_sheet.png", graphics).await,
            platform_texture: Images::load_from_file("platform_tile_sheet.png", graphics).await,
        }
    }

    async fn load_from_file(filename: &str, graphics: &Graphics) -> Image {
        Image::load(&graphics, filename)
            .await
            .expect(&format!("Couldn't load file: {}", filename))
    }
}
