use crossterm::{ self, };
use std::io::{ stdout, Write, };

pub fn on_backspace( initial_cur_x: u16, ) {
    if initial_cur_x > 0 {
        cursor_x -= 1;
        term::move_cursor( cursor_x, cursor_y ).unwrap();
        write!( stdout,  " " ).unwrap();
        // so that the cursor doesn't lag a box behind
        term::move_cursor( cursor_x, cursor_y ).unwrap();
    }
}

pub fn on_input() {
    // ...
}

pub fn on_quit() {
    // ...
}
