use std::{ 
    self,
    io::{
        self,
        Read, Write,
        stdin, stdout
    },
};
use crate::term::{ self, TermSize };

pub fn run() -> Result<(), io::Error> {
    // Actual run code goes here

    let mut stdin = stdin();
    let mut stdout = stdout();
    let term_size: TermSize = term::get_term_size()?;
    println!( "Term size: {:?}", term_size );
    println!( "" );

    let mut char_index = 0;

    term::enter_raw_mode();

    for b in stdin.bytes() {
        let b = b.unwrap();
        let c = b as char;
        // println!(  "{}", c );

        term::move_cursor( &stdout, 0, term_size.cols - 2 );

        if c.is_control() {
            // all the spaces are to remove any `Char: {}` that was there previously
            print!( "Binary: {0:08b}  ASCII: {0:#3?}              \r", b );
        } else if c != 'q' {
            print!( "Binary: {0:08b}  ASCII: {0:#3?}  Char: {1:#?}\r", b, c );
        }

        term::move_cursor( &stdout, char_index, term_size.cols );
        if c != 'q' { print!( "{}", c ); }
        char_index += 1;

        if c == 'q' {
            println!( "\n\rQuitting... \r" );
            break;
        };
    };

    term::exit_raw_mode();
    Ok(())
}