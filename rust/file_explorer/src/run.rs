use std::{
    self,
    io::{
        self,
        Read, Write,
        stdin, stdout
    },
};
use crossterm::execute;
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color, Attribute};
use crate::debug;
use crate::handle;
use crate::term::{ self, TermSize, PathData, PathType };

use crate::MAIN_SECTION_X;

/// Runs base checks for args etc. before confirming that `_run()` can be used
pub fn run() -> Result<(), Box<dyn std::error::Error>>{
    let _args: Vec<_> = std::env::args().collect();

    let term_size = term::get_term_size().unwrap();

    if term_size.rows < 6 {
        println!( "Terminal too small!" );
        println!(
            "    Terminal must have at least 6 rows. ( yours has {} )",
            term_size.rows
        );
    } else if term_size.cols < 30 {
        println!( "Terminal too small!" );
        println!(
            "    Terminal must have at least 30 rows. ( yours has {} )",
            term_size.cols
        );
    } else {
        _run().unwrap();
        return Ok(());
    }
    Err( "Bad terminal size".into() )
}

/// Actual run functiom, including main loop
pub fn _run() -> Result<(), io::Error> {
    let stdin = stdin();
    let mut screen = stdout();
    let term_size: TermSize = term::get_term_size()?;

    term::enter_raw_mode( &screen );
    term::hide_cursor( &screen );
    
    let dir: std::fs::ReadDir = std::fs::read_dir(".").unwrap();
    let mut path_count = 0;
    let mut paths: Vec<PathData> = vec![];
    for path in dir {
        let path = path.unwrap().path();
        
        term::move_cursor( &screen, MAIN_SECTION_X, path_count );
        
        write!( screen, "{:<20}", path.display() ).unwrap();
        screen.flush().unwrap();
        
        path_count += 1;
        paths.push( PathData::new( path ) );
    }
 
    handle::select( &screen, &paths, MAIN_SECTION_X, 0 );

    let mut selected_index = 0;

    std::thread::sleep( std::time::Duration::from_millis( 500 ) );
    
    'main: loop {
        break;
    }

    for byte in stdin.bytes() {
        let byte = byte?;
        let c = byte as char;

        term::move_cursor( &screen, MAIN_SECTION_X, 0 );
        screen.flush().unwrap();

        match c {
            'h' => { todo!() }
            'j' => {
                if selected_index < path_count - 1 {
                    handle::select_down( &screen, &paths, MAIN_SECTION_X, selected_index );
                    selected_index += 1;
                }
            }
            'k' => {
                if selected_index > 0 {
                    handle::select_up( &screen, &paths, MAIN_SECTION_X, selected_index );
                    selected_index -= 1;
                }
            }
            'l' => { todo!() }
            'q' => {
                handle::on_quit( &screen, term_size.cols );
                break;
            }
            _ => {
                handle::on_input( &screen, c );
            }
        };
    }

    term::exit_raw_mode( &screen );
    term::show_cursor( &screen );
    Ok(())
}
