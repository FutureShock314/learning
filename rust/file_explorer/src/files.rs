use std::io::{ Stdout, Write };
use std::path::PathBuf;
use crossterm::execute;
use crossterm::style::{
    self,
    Print,
    ResetColor,
    SetForegroundColor,
    Attribute, SetAttribute
};
use crate::term;
use crate::MAIN_SECTION_FRAC;

#[ derive( PartialEq ) ]
pub enum PathType {
    File,
    Dir,
}

pub struct PathData {
    pub path: std::path::PathBuf,
    pub path_type: PathType,
    pub col_1: style::Color,
    pub col_2: style::Color,
}

impl PathData {
    pub fn new( path: std::path::PathBuf ) -> PathData {
        if path.is_dir() {
            let path_type = PathType::Dir;
            let col_1 = style::Color::Blue;
            let col_2 = style::Color::Black;

            PathData {
                path,
                path_type,
                col_1,
                col_2
            }
        } else {
            let path_type = PathType::File;
            // let col_1 = style::Color::Rgb { r: 255, g: 255, b: 255 };
            let col_1 = style::Color::Grey;
            let col_2 = style::Color::Black;

            PathData {
                path,
                path_type,
                col_1,
                col_2
            }
        }
    }
}


fn path_name( path: PathBuf ) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn files_section( mut screen: &Stdout, path: PathBuf, x: u16 ) -> Vec<PathData> {
    let term_size = term::get_term_size().unwrap();
    let cols = term_size.cols as f64;
    let width: usize = ( cols * MAIN_SECTION_FRAC ).floor() as usize;

    let dir = std::fs::read_dir( path ).unwrap();

    let mut paths: Vec<PathData> = vec![];
    let mut files: Vec<PathData> = vec![];
    let mut dirs : Vec<PathData> = vec![];

    clear_area( screen, 0, x, term_size.rows - 1, term_size.cols - 1 );

    term::move_cursor( screen, x, term_size.cols - 1 );
    term::clear_line( screen );

    for path in dir {
        let path = PathData::new( path.unwrap().path() );
        match path.path_type {
            PathType::Dir => { dirs.push( path ); }
            PathType::File => { files.push( path ); }
        };
    }

    // !
    // ! This is really performance heavy and I should really make it
    // ! more efficient at some point
    // !

    dirs.sort_by(
        | a, b |
        path_name( a.path.clone() ).cmp( &path_name( b.path.clone() ) )
    );
    
    files.sort_by(
        | a, b |
        path_name( a.path.clone() ).cmp( &path_name( b.path.clone() ) )
    );

    paths.extend( dirs );
    paths.extend( files );
    
    for ( path_index, path  ) in paths.iter().enumerate() {
        term::move_cursor( screen, x, path_index as u16 );
        execute!(
            screen,
            SetForegroundColor( path.col_1 ),
            SetAttribute( Attribute::Bold ),
            Print( format!(
                " {:<1$}",
                path.path.file_name().unwrap().to_str().unwrap(),
                width,
            ) ),
            ResetColor,
            SetAttribute( Attribute::Reset )
        ).unwrap();
    }
    screen.flush().unwrap();

    paths
}

fn clear_area(
    mut screen: &Stdout,
    top: u16,
    left: u16,
    bottom: u16,
    right: u16,
) {
    // Precompute the spaces string for efficiency
    let width = right - left + 1;
    let spaces = " ".repeat(width as usize);

    // Clear each row in the target area
    for row in top..=bottom {
        term::move_cursor( screen, left, row );
        write!( screen, "{}", &spaces ).unwrap();
    }
}
