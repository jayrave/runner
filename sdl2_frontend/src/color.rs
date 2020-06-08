use runner_core::color::Color;
use sdl2::pixels::Color as SdlColor;

/// Instead of `Into` doing it this way since both the trait
/// & the struct are from outside this crate & so Rust wouldn't
/// allow this
pub fn sdl_color_from(color: Color) -> SdlColor {
    SdlColor::RGBA(color.red(), color.green(), color.blue(), color.alpha())
}
