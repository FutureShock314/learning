use std::io::{ Stdout, Write };
use std::path::PathBuf;
use crossterm::execute;
use crossterm::style::{
    Print,
    ResetColor,
    SetForegroundColor,
    Attribute, SetAttribute
};
use crate::term::{ self, PathData };
use crate::MAIN_SECTION_FRAC;

pub fn main_section_files( mut screen: &Stdout, path: PathBuf, x: u16 ) -> Vec<PathData> {
    let term_size = term::get_term_size().unwrap();
    let cols = term_size.cols as f64;
    let width: usize = ( cols * MAIN_SECTION_FRAC ).floor() as usize;

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