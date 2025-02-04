use crossterm::{ 
    terminal::{ self, /*Clear,*/ size },
    /*ExecutableCommand,*/ execute,
    cursor::{ MoveTo },
};
use std::{
    io::{ self, Stdout, stdout, /*Read,*/ },
    error::Error,
};

#[ derive( Debug ) ]
pub struct TermSize {
    pub cols: u16,
    pub rows: u16,
}

// !
// ! ALWAYS CALL `exit_raw_mode()` AT END OF CODE
// !

pub fn enter_raw_mode( mut stdout: &Stdout ) {
    execute!( stdout, terminal::EnterAlternateScreen );
    terminal::enable_raw_mode().ok();
}

pub fn exit_raw_mode( mut stdout: &Stdout ) {
    execute!( stdout, terminal::LeaveAlternateScreen );
    terminal::disable_raw_mode().ok();
}

pub fn clear_screen( stdout: &Stdout ) {
    //
}

pub fn move_cursor( mut stdout: &Stdout, x: u16, y: u16 ) -> Result<(), Box<dyn Error>> {
    execute!( stdout, MoveTo( x, y ) )?;
    Ok(())
}

pub fn get_term_size() -> Result<TermSize, io::Error> {
    let ( cols, rows ) = size()?;
    Ok(TermSize { cols, rows })
}
