pub fn colored(color: &Color, text: &str) -> String {
    return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", color.r, color.g, color.b, text);
}

pub fn pattern_color( mut text: String, pattern: &str, color: &Color ) -> String {
    let idx: usize = text.to_lowercase().find( pattern ).unwrap().try_into().unwrap();

    let ptrn_len: usize = pattern.len().try_into().unwrap();
    let idx2: usize = idx + ptrn_len;

    let original_pattern_occurrance = &text[ idx..idx2 ];

    text.replace_range(
        idx..idx2,
        &colored( color, original_pattern_occurrance )
    );

    text
}

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
