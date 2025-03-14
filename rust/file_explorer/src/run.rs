use std::{
    self, io::{
        self, stdin, stdout, Read, Write
    }, path::PathBuf
};
use crossterm::style::{
    SetAttribute, Attribute,
    SetBackgroundColor, Color, ResetColor,
    Print
};
// use crate::debug;
use crate::files;
use crate::handle;
use crate::term::{ self, TermSize };

use crate::MAIN_SECTION_X;

/// Runs base checks for args etc. before confirming that `_run()` can be used
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();

    // println!( "{:?}", args );

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
        }
        
        return Ok(());
    }
    Err( "Bad terminal size".into() )
}

/// Actual run function, including main loop
pub fn _run( init_path: PathBuf ) -> Result<(), io::Error> {
    let stdin = stdin();
    let mut screen = stdout();
    let term_size: TermSize = term::get_term_size()?;

    // so that it can be muted later
    let mut path = std::path::absolute( init_path ).unwrap();

    term::enter_raw_mode( &screen );
    term::hide_cursor( &screen );
    
    let mut paths =
        files::files_section( &screen, path.clone(), MAIN_SECTION_X );
    let mut path_count = paths.len();
 
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

        match ( c, byte ) {
            ( 'h', _ ) => {
                let old_path = path.pop();
                let old_path = files::PathData::new( old_path );
                let old_index = paths.iter().position(
                    | x | x == old_path
                ).unwrap();

                paths =
                    files::files_section( &screen, path.clone(), MAIN_SECTION_X )
                ;
                path_count = paths.len();
                handle::select( &screen, &paths, MAIN_SECTION_X, 0 );
                selected_index = 0;
            }
            ( 'j', _ ) => {
                if selected_index < path_count - 1 {
                    handle::select_down( &screen, &paths, MAIN_SECTION_X,
                        selected_index.try_into().unwrap()
                    );
                    selected_index += 1;
                }
            }
            ( 'k', _ ) => {
                if selected_index > 0 {
                    handle::select_up( &screen, &paths, MAIN_SECTION_X,
                        selected_index.try_into().unwrap()
                    );
                    selected_index -= 1;
                }
            }
            ( 'l', _ ) => {
                if paths[selected_index].path_type == files::PathType::Dir {
                    path = paths[selected_index].path.clone();
                    paths =
                        files::files_section( &screen, path.clone(), MAIN_SECTION_X )
                    ;
                    path_count = paths.len();
                    handle::select( &screen, &paths, MAIN_SECTION_X, 0 );
                    selected_index = 0;
                }
            }
            ( 'q', _ ) => {
                handle::on_quit( &screen, term_size.cols );
                break;
            }
            ( _,  13 ) => {
                if paths[selected_index].path.is_dir() {
                    let temp = std::env::temp_dir().join( "file_explorer" );
                    std::fs::write(
                        temp,
                        format!( "{}", paths[selected_index].path.display() )
                    ).unwrap();

                    break;
                } else {
                    term::move_cursor( &screen, 0, term_size.cols - 1 );
                    term::clear_line( &screen );
                    crossterm::execute!(
                        &screen,
                        SetBackgroundColor( Color::Red ),
                        SetAttribute( Attribute::Bold ),
                        Print( " Not a directory! " ),
                        ResetColor,
                        SetAttribute( Attribute::Reset )
                    ).unwrap();
                    screen.flush().unwrap();
                }
            }
            _ => { continue; }
        };
    }

    term::exit_raw_mode( &screen );
    term::show_cursor( &screen );
    Ok(())
}
