use std::io::{ Stdout, Write };
use std::path::PathBuf;
use crossterm::execute;
use crossterm::style::{
    Print,
    ResetColor,
    SetForegroundColor,
    Attribute, SetAttribute
};
use crate::term::{ self, PathData, PathType::{ Dir, File } };
use crate::MAIN_SECTION_FRAC;

fn path_name( path: PathBuf ) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn main_section_files( mut screen: &Stdout, path: PathBuf, x: u16 ) -> Vec<PathData> {
    let term_size = term::get_term_size().unwrap();
    let cols = term_size.cols as f64;
    let width: usize = ( cols * MAIN_SECTION_FRAC ).floor() as usize;

    let dir = std::fs::read_dir( path ).unwrap();

    let mut paths: Vec<PathData> = vec![];
    let mut files: Vec<PathData> = vec![];
    let mut dirs : Vec<PathData> = vec![];

    for path in dir {
        let path = PathData::new( path.unwrap().path() );
        match path.path_type {
            Dir => { dirs.push( path ); }
            File => { files.push( path ); }
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
    
    let mut path_index = 0;

    for path in &paths {
        term::move_cursor( screen, x, path_index );
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
        path_index += 1;
    }
    screen.flush().unwrap();

    paths
}