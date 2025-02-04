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
use std::{
    io::{ self, Stdout, stdout, /*Read,*/ Write },
    error::Error,
};

pub enum PathType {
    File,
    Dir,
}

#[ derive( Debug ) ]
pub struct TermSize {
    pub cols: u16,
    pub rows: u16,
}

pub struct PathData {
    pub path: std::path::PathBuf,
    pub path_type: PathType,
    pub bg_col: style::Color,
    pub fg_col: style::Color,
}

impl PathData {
    pub fn new( path: std::path::PathBuf ) -> PathData {
        if path.is_dir() {
            let path_type = PathType::Dir;
            let bg_col = style::Color::Blue;
            let fg_col = style::Color::Black;
            return PathData {
                path: path,
                path_type: path_type,
                bg_col: bg_col,
                fg_col: fg_col
            };
        } else {
            let path_type = PathType::File;
            let bg_col = style::Color::White;
            let fg_col = style::Color::Black;
            return PathData {
                path: path,
                path_type: path_type,
                bg_col: bg_col,
                fg_col: fg_col
            };
        }
    }
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

pub fn move_cursor( mut screen: &Stdout, x: u16, y: u16 ) -> Result<(), Box<dyn Error>> {
    execute!( screen, MoveTo( x, y ) );
    Ok(())
}

/// Returns a `TermSize` object containing `{ cols, rows }`
pub fn get_term_size() -> Result<TermSize, io::Error> {
    let ( cols, rows ) = size()?;
    Ok(TermSize { cols, rows })
}
