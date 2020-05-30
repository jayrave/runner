#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }
}
