use crossterm::{ 
    terminal::{ self, /*Clear,*/ size },
    /*ExecutableCommand,*/ execute,
    cursor::{ MoveTo, Hide, Show },
};
use std::{
    io::{ self, Stdout, stdout, /*Read,*/ Write },
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

pub fn enter_raw_mode( mut screen: &Stdout ) {
    execute!( screen, terminal::EnterAlternateScreen ).ok();
    terminal::enable_raw_mode().ok();
}

pub fn hide_cursor( mut screen: &Stdout ) {
    execute!( screen, Hide ).unwrap();
}

pub fn exit_raw_mode( mut screen: &Stdout ) {
    execute!( screen, terminal::LeaveAlternateScreen ).ok();
    terminal::disable_raw_mode().ok();
}

pub fn show_cursor( mut screen: &Stdout ) {
    execute!( screen, Show ).unwrap();
}

// pub fn clear_screen( stdout: &Stdout ) {
//     //
// }

pub fn move_cursor( mut screen: &Stdout, x: u16, y: u16 ) -> Result<(), Box<dyn Error>> {
    execute!( screen, MoveTo( x, y ) );
    Ok(())
}

/// Returns a `TermSize` object containing `{ cols, rows }`
pub fn get_term_size() -> Result<TermSize, io::Error> {
    let ( cols, rows ) = size()?;
    Ok(TermSize { cols, rows })
}
