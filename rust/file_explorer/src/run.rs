use std::{ 
    self,
    io::{
        self,
        Read, Write,
        stdin, stdout
    },
};
use crate::term;

pub fn run() -> Result<(), io::Error> {
    // Actual run code goes here

    let mut stdin = stdin();
    let mut stdout = stdout();

    term::enter_raw_mode();

    for b in stdin.bytes() {
        let b = b.unwrap();
        let c = b as char;
        // println!(  "{}", c );

        term::move_cursor( &stdout );

        if c.is_control() {
            println!( "Binary: {0:08b}  ASCII: {0:#3?}\r", b );
        } else {
            println!( "Binary: {0:08b}  ASCII: {0:#3?}  Char: {1:#?}\r", b, c );
        }
        if c == 'q' {
            break;
        };
    };

    term::exit_raw_mode();
    Ok(())
}