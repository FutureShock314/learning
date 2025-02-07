use crossterm::{ self, execute };
use crossterm::style::{
    Print,
    SetForegroundColor, SetBackgroundColor,
    ResetColor, Color, Attribute, SetAttribute
};
use std::io::{ Stdout, Write, };
use crate::term::{ self, PathData, };
use crate::MAIN_SECTION_FRAC;

fn fg( col: Color ) -> SetForegroundColor {
    SetForegroundColor( col )
}

fn bg( col: Color ) -> SetBackgroundColor {
    SetBackgroundColor( col )
}

pub fn on_quit( mut screen: &Stdout, cols: u16 ) {
    term::move_cursor( screen, 0, cols - 1 );
    term::clear_line( screen );
    // write!( screen, "Quitting..." ).unwrap();
    execute!(
        screen,
        bg( Color::Red ),
        SetAttribute( Attribute::Bold ),
        Print( " Quitting... " ),
        ResetColor,
        SetAttribute( Attribute::Reset )
    ).unwrap();
    screen.flush().unwrap();
    std::thread::sleep( std::time::Duration::from_millis( 500 ) );
}

pub fn select( mut screen: &Stdout, paths: &Vec<PathData>, x: u16, index: u16, ) {
    let term_size = term::get_term_size().unwrap();
    let cols = term_size.cols as f64;
    let width: usize = ( cols * MAIN_SECTION_FRAC ).floor() as usize;

    term::move_cursor( screen, x, index );
    let index = index as usize;
    let path = &paths[index];
    term::clear_line( screen );
    execute!(
        screen,
        SetAttribute( Attribute::Bold ),
        bg( path.col_1 ), fg( path.col_2 ),
        Print( format!(
            " {:<1$} {1:}",
            path.path.file_name().unwrap().to_str().unwrap(),
            width,
        ) ),
        SetAttribute( Attribute::Reset ),
        ResetColor,
    ).unwrap();
}

pub fn deselect( mut screen: &Stdout, paths: &Vec<PathData>, x: u16, index: u16, ) {
    let term_size = term::get_term_size().unwrap();
    let cols = term_size.cols as f64;
    let width: usize = ( cols * MAIN_SECTION_FRAC ).floor() as usize;

    term::move_cursor( screen, x, index );
    let index = index as usize;
    let path = &paths[index];
    term::clear_line( screen );
    execute!(
        screen,
        SetAttribute( Attribute::Bold ),
        fg( path.col_1 ),
        Print( format!(
            " {:<1$}",
            path.path.file_name().unwrap().to_str().unwrap(),
            width,
        ) ),
        SetAttribute( Attribute::Reset ),
        ResetColor,
    ).unwrap();
}

pub fn select_up( screen: &Stdout, paths: &Vec<PathData>, x: u16, curr_selected_index: u16 ) {
    deselect( screen, paths, x, curr_selected_index );
    select(   screen, paths, x, curr_selected_index - 1 );
}

pub fn select_down( screen: &Stdout, paths: &Vec<PathData>, x: u16, curr_selected_index: u16 ) {
    deselect( screen, paths, x, curr_selected_index );
    select(   screen, paths, x, curr_selected_index + 1 );
}
