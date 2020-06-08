use quicksilver::graphics::Image;
use quicksilver::Graphics;

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
    pub async fn load_from_files(graphics: &Graphics) -> Images {
        let mut images = futures::future::join_all(vec![
            Images::load_from_file("cloud_tile_sheet.png", graphics),
            Images::load_from_file("character_tile_sheet.png", graphics),
            Images::load_from_file("enemy_tile_sheet.png", graphics),
            Images::load_from_file("icon_tile_sheet.png", graphics),
            Images::load_from_file("letter_tile_sheet.png", graphics),
            Images::load_from_file("platform_tile_sheet.png", graphics),
            Images::load_from_file("number_tile_sheet.png", graphics),
        ]).await.into_iter();

        Images {
            cloud_texture: images.next().unwrap(),
            character_texture: images.next().unwrap(),
            enemy_texture: images.next().unwrap(),
            icon_texture: images.next().unwrap(),
            letter_texture: images.next().unwrap(),
            platform_texture: images.next().unwrap(),
            number_texture: images.next().unwrap(),
        }
    }

    async fn load_from_file(filename: &str, graphics: &Graphics) -> Image {
        Image::load(&graphics, filename)
            .await
            .unwrap_or_else(|_| panic!("Couldn't load file: {}", filename))
    }
}
