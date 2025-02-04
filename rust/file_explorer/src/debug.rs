use crate::term;

pub fn check_byte( byte: u16, c: char, loc_x: u16, loc_y: u16 ) {
    if c.is_control() {
        // all the spaces are to remove any `Char: {}` that was there previously
        write!( stdout, "Binary: {0:08b}  ASCII: {0:#3?}              \r", byte ).unwrap();
    } else if c != 'q' {
        write!( stdout, "Binary: {0:08b}  ASCII: {0:#3?}  Char: {1:#?}\r", byte, c ).unwrap();
    }
}