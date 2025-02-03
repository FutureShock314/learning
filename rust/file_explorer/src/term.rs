use crossterm::terminal::{ self, Clear };

// 
// ALWAYS CALL `exit_raw_mode()` AT END OF CODE
//

pub fn enter_raw_mode() {
    terminal::enable_raw_mode().ok();
}

pub fn exit_raw_mode() {
    terminal::disable_raw_mode().ok();
}
