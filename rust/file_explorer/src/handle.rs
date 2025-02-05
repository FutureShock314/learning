use crossterm::{ self, execute };
use crossterm::style::{
    Print,
    SetForegroundColor, SetBackgroundColor,
    ResetColor, Color, Attribute, SetAttribute, Stylize
};
use std::io::{ Stdout, stdout, Write, };
use crate::term::{ self, PathData, };

fn fg( col: Color ) -> SetForegroundColor {
    SetForegroundColor( col )
}

fn bg( col: Color ) -> SetBackgroundColor {
    SetBackgroundColor( col )
}

pub fn on_quit( mut screen: &Stdout, cols: u16 ) {
    term::move_cursor( screen, 0, cols - 1 );
    term::clear_line( screen );
    write!( screen, "Quitting..." ).unwrap();
    screen.flush().unwrap();
    std::thread::sleep( std::time::Duration::from_millis( 500 ) );
}

pub fn select( mut screen: &Stdout, paths: &Vec<PathData>, x: u16, index: u16, ) {
    term::move_cursor( screen, x, index );
    let index = index as usize;
    let path = &paths[index];
    term::clear_line( screen );
    execute!(
        screen,
        SetAttribute( Attribute::Bold ),
        bg( path.col_1 ), fg( path.col_2 ),
        Print( format!(
            "{:<20}",
            path.path.display()
        ) ),
        SetAttribute( Attribute::Reset ),
        ResetColor,
    ).unwrap();
}

pub fn deselect( mut screen: &Stdout, paths: &Vec<PathData>, x: u16, index: u16, ) {
    term::move_cursor( screen, x, index );
    let index = index as usize;
    let path = &paths[index];
    term::clear_line( screen );
    execute!(
        screen,
        SetAttribute( Attribute::Bold ),
        fg( path.col_1 ),
        Print( format!(
            "{:<20}",
            path.path.display()
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
