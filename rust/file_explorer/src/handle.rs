use crossterm::{
    self, execute,
    terminal::{ Clear, ClearType, }
};
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color, Attribute};
use std::io::{ stdout, Stdout, Write, };
use crate::term;

pub fn on_backspace( mut stdout: &Stdout, mut cursor_x: u16, cursor_y: u16, min_x: u16 ) -> u16 {
    if cursor_x > min_x {
        cursor_x -= 1;
        term::move_cursor( stdout, cursor_x, cursor_y ).unwrap();
        write!( stdout,  " " ).unwrap();
        // so that the cursor doesn't lag a box behind
        term::move_cursor( stdout, cursor_x, cursor_y ).unwrap();
    }
    cursor_x
}

pub fn on_input( stdout: &Stdout, input: char ) {
    // ...
}

pub fn on_quit( mut stdout: &Stdout, cols: u16 ) {
    term::move_cursor( stdout, 0, cols - 1 ).unwrap();
    write!( stdout, "Quitting..." ).unwrap();
    stdout.flush().unwrap();
    std::thread::sleep( std::time::Duration::from_millis( 500 ) );
}

pub fn select( mut screen: &Stdout, paths: Vec<std::path::PathBuf>, indentation: u16, index: u16, ) {
    term::move_cursor( screen, indentation, index ).unwrap();
    let index = index as usize;
    execute!( screen, Clear( ClearType::CurrentLine ) );
    execute!(
        screen,
        SetBackgroundColor( Color::Blue ),
        SetForegroundColor( Color::Black ),
        Print( format!(
            "{:<20}",
            paths[index].display()
        ) ),
        ResetColor,
    );
}
