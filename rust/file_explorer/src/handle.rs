use crossterm::{
    self, execute,
    terminal::{ Clear, ClearType, }
};
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color, Attribute};
use std::io::{ stdout, Stdout, Write, };
use crate::term::{ self, PathData, PathType, };

pub fn on_backspace( mut screen: &Stdout, mut cursor_x: u16, cursor_y: u16, min_x: u16 ) -> u16 {
    if cursor_x > min_x {
        cursor_x -= 1;
        term::move_cursor( screen, cursor_x, cursor_y ).unwrap();
        write!( stdout,  " " ).unwrap();
        // so that the cursor doesn't lag a box behind
        term::move_cursor( screen, cursor_x, cursor_y ).unwrap();
    }
    cursor_x
}

pub fn on_input( stdout: &Stdout, input: char ) {
    // ...
}

pub fn on_quit( mut screen: &Stdout, cols: u16 ) {
    term::move_cursor( screen, 0, cols - 1 ).unwrap();
    write!( screen, "Quitting..." ).unwrap();
    stdout.flush().unwrap();
    std::thread::sleep( std::time::Duration::from_millis( 500 ) );
}

pub fn select( mut screen: &Stdout, paths: &Vec<PathData>, x: u16, index: u16, ) {
    term::move_cursor( screen, x, index ).unwrap();
    let index = index as usize;
    let path = paths[index];
    term::clear_line( screen );
    execute!(
        screen,
        SetBackgroundColor( path.bg_col ),
        SetForegroundColor( path.fg_col ),
        Print( format!(
            "{:<20}",
            path.path.display()
        ) ),
        ResetColor,
    );
}

pub fn deselect( mut screen: &Stdout, paths: &Vec<PathData>, x: u16, index: u16, ) {
    term::move_cursor( screen, x, index ).unwrap();
    let index = index as usize;
    let path = paths[index];
    term::clear_line( screen );
    write!( screen, "{:<20}", path.path.display() ).unwrap();
    screen.flush().unwrap();
}

pub fn select_up( screen: &Stdout, paths: Vec<PathData>, x: u16, curr_selected_index: u16 ) {
    deselect( screen, &paths, x, curr_selected_index );
    select(   screen, &paths, x, curr_selected_index - 1 );
}

pub fn select_down( screen: &Stdout, paths: Vec<PathData>, x: u16, curr_selected_index: u16 ) {
    deselect( screen, &paths, x, curr_selected_index );
    select(   screen, &paths, x, curr_selected_index + 1 );
}
