use crossterm::{ self, };
use std::io::{ stdout, Write, };
use crate::term;

pub fn on_backspace( mut cursor_x: u16, cursor_y: u16, min_x: u16 ) -> u16 {
    let mut stdout = stdout();

    if cursor_x > min_x {
        cursor_x -= 1;
        term::move_cursor( cursor_x, cursor_y ).unwrap();
        write!( stdout,  " " ).unwrap();
        // so that the cursor doesn't lag a box behind
        term::move_cursor( cursor_x, cursor_y ).unwrap();
    }
    cursor_x
}

pub fn on_input() {
    // ...
}

pub fn on_quit() {
    // ...
}
