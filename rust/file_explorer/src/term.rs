use crossterm::{ 
    terminal::{ self, Clear },
    ExecutableCommand, execute,
    cursor::{ MoveTo },
};
use std::io::{ Stdout, Read };

// 
// ALWAYS CALL `exit_raw_mode()` AT END OF CODE
//

pub fn enter_raw_mode() {
    terminal::enable_raw_mode().ok();
}

pub fn exit_raw_mode() {
    terminal::disable_raw_mode().ok();
}

pub fn move_cursor( mut term: &Stdout, /* x: i32, y: i32 */ ) {
    execute!( term, MoveTo( 10, 20 ) );
}
