use std::{
    self,
    io::{
        self,
        Read, Write,
        stdin, stdout
    },
    sync::{ Mutex, Arc },
    path::PathBuf
};
// use crate::debug;
use crate::files;
use crate::handle;
use crate::term::{ self, TermSize, PathData, };

use crate::MAIN_SECTION_X;

/// Runs base checks for args etc. before confirming that `_run()` can be used
pub fn run() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<_> = std::env::args().collect();

    println!( "{:?}", args );

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
        if args.len() > 1 {
            let path = PathBuf::from( args[1].clone() );
                // .expect( "Invalid Path Argument!" );
            _run( path ).unwrap();
        } else {
            _run( PathBuf::from( "." ) ).unwrap();
        
        return Ok(());
    }
    Err( "Bad terminal size".into() )
}

/// Actual run functiom, including main loop
pub fn _run( init_path: PathBuf ) -> Result<(), io::Error> {
    let stdin = stdin();
    let mut screen = stdout();
    let term_size: TermSize = term::get_term_size()?;

    term::enter_raw_mode( &screen );
    term::hide_cursor( &screen );
    
    let paths = files::main_section_files( &screen, init_path, MAIN_SECTION_X );
    let path_count = paths.len();
 
    handle::select( &screen, &paths, MAIN_SECTION_X, 0 );

    let mut selected_index = 0;

    std::thread::sleep( std::time::Duration::from_millis( 500 ) );
    
    'main: loop {
        break 'main;
    }

    for byte in stdin.bytes() {
        let byte = byte?;
        let c = byte as char;

        term::move_cursor( &screen, MAIN_SECTION_X, 0 );
        screen.flush().unwrap();

        match c {
            'h' => { continue; }
            'j' => {
                if selected_index < path_count - 1 {
                    handle::select_down( &screen, &paths, MAIN_SECTION_X,
                        selected_index.try_into().unwrap()
                    );
                    selected_index += 1;
                }
            }
            'k' => {
                if selected_index > 0 {
                    handle::select_up( &screen, &paths, MAIN_SECTION_X,
                        selected_index.try_into().unwrap()
                    );
                    selected_index -= 1;
                }
            }
            'l' => { continue; }
            'q' => {
                handle::on_quit( &screen, term_size.cols );
                break;
            }
            _ => {
                continue;
            }
        };
    }

    term::exit_raw_mode( &screen );
    term::show_cursor( &screen );
    Ok(())
}
