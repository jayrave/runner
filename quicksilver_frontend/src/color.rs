use quicksilver::graphics::Color as QsColor;
use runner_core::color::Color;

/// Instead of `Into` doing it this way since both the trait
/// & the struct are from outside this crate & so Rust wouldn't
/// allow this
pub fn qs_color_from(color: Color) -> QsColor {
    QsColor::from_rgba(
        color.red(),
        color.green(),
        color.blue(),
        color.alpha() as f32 / u8::max_value() as f32,
    )
}
