use std::io::{ stdout, Write };
use crate::term;

pub fn check_byte( byte: u8, c: char, loc_x: u16, loc_y: u16 ) {
    let mut stdout = stdout();

    term::move_cursor( loc_x, loc_y );

    if c.is_control() {
        // all the spaces are to remove any `Char: {}` that was there previously
        write!( stdout, "Binary: {0:08b}  ASCII: {0:#3?} {1:─<14}\r", byte, '─' ).unwrap();
    } else if c != 'q' {
        write!( stdout, "Binary: {0:08b}  ASCII: {0:#3?}  Char: {1:#?}\r", byte, c ).unwrap();
    }
}