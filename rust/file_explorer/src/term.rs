use crossterm::{ 
    terminal::{
        self,
        Clear, ClearType,
        size
    },
    /*ExecutableCommand,*/ execute,
    cursor::{ MoveTo, Hide, Show },
    style,
};
use std::io::{ self, Stdout, };

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

pub fn clear_line( mut screen: &Stdout ) {
    execute!( screen, Clear( ClearType::CurrentLine ) ).unwrap();
}

pub fn move_cursor( mut screen: &Stdout, x: u16, y: u16 ) {
    execute!( screen, MoveTo( x, y ) ).unwrap();
}

/// Returns a `TermSize` object containing `{ cols, rows }`
pub fn get_term_size() -> Result<TermSize, io::Error> {
    let ( cols, rows ) = size()?;
    Ok(TermSize { cols, rows })
}
