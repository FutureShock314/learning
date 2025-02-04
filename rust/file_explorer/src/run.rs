use std::{
    self,
    io::{
        self,
        Read, Write,
        stdin, stdout
    },
};
use crate::debug;
use crate::handle;
use crate::term::{ self, TermSize };

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

    term::move_cursor( &screen, 5, 0 ).unwrap();

    let paths = std::fs::read_dir("./").unwrap();
    for path in paths {
        write!( screen, "{}\n\r", path.unwrap().path().display() ).unwrap();
    }
    screen.flush().unwrap();
    
    'main: loop {
        break;
    }

    term::exit_raw_mode( &screen );
    Ok(())
}
