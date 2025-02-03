use crossterm::{ 
    terminal::{ self, /*Clear,*/ size },
    /*ExecutableCommand,*/ execute,
    cursor::{ MoveTo },
};
use std::{
    io::{ self, Stdout, /*Read,*/ },
    error::Error,
};

#[ derive( Debug ) ]
pub struct TermSize {
    pub rows: u16,
    pub cols: u16,
}

// 
// ALWAYS CALL `exit_raw_mode()` AT END OF CODE
//

pub fn enter_raw_mode() {
    terminal::enable_raw_mode().ok();
}

pub fn exit_raw_mode() {
    terminal::disable_raw_mode().ok();
}

pub fn move_cursor( mut term: &Stdout, x: u16, y: u16 ) -> Result<(), Box<dyn Error>> {
    execute!( term, MoveTo( x, y ) )?;
    Ok(())
}

pub fn get_term_size() -> Result<TermSize, io::Error> {
    let ( rows, cols ) = size()?;
    Ok(TermSize { rows, cols })
}
