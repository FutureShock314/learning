use std::io::{ stdout, Stdout, Write };
use crate::term;

pub fn _check_byte( mut stdout: &Stdout, byte: u8, c: char, loc_x: u16, loc_y: u16 ) {
    term::move_cursor( stdout, loc_x, loc_y ).unwrap();

    if c.is_control() {
        // all the spaces are to remove any `Char: {}` that was there previously
        write!( stdout, "┤ Binary: {0:08b}  ASCII: {0:#3?} ├{1:─<14}\r", byte, '─' ).unwrap();
    } else if c != 'q' {
        write!( stdout, "┤ Binary: {0:08b}  ASCII: {0:#3?}  Char: {1:#?} ├\r", byte, c ).unwrap();
    }
}