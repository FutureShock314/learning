#[ derive( Debug ) ]
pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

impl Color {
    pub fn new( r: i32, g: i32, b: i32 ) -> Color {
        Color { r, g, b }
    }
}
pub fn colored(color: &Color, text: &str) -> String {
    return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", color.r, color.g, color.b, text);
}
