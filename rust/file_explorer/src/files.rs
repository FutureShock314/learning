use std::io::{ Stdout, Write };
use std::path::PathBuf;
use crossterm::execute;
use crossterm::style::{
    Print,
    Color, ResetColor,
    SetForegroundColor,
    Attribute, SetAttribute
};
use crate::term::{ self, PathData };

pub fn main_section_files( mut screen: &Stdout, path: PathBuf, x: u16 ) -> Vec<PathData> {
    let dir = std::fs::read_dir( path ).unwrap();
    let mut paths: Vec<PathData> = vec![];
    for path in dir {
        paths.push( PathData::new( path.unwrap().path() ) );
    }
    let mut path_index = 0;

    for path in &paths {
        term::move_cursor( screen, x, path_index );
        execute!(
            screen,
            SetForegroundColor( path.col_1 ),
            SetAttribute( Attribute::Bold ),
            Print( format!( "{:<20}", path.path.display() ) ),
            ResetColor,
            SetAttribute( Attribute::Reset )
        ).unwrap();
        path_index += 1;
    }
    screen.flush().unwrap();

    paths
}