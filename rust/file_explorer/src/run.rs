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
    let stdin = stdin();
    let mut stdout = stdout();
    let term_size: TermSize = term::get_term_size()?;
    println!( "Terminal size: {:?}", term_size );
    println!( "" );

    let mut cursor_x = 0;

    term::enter_raw_mode();

    for byte in stdin.bytes() {
        let byte = byte.unwrap(); // would use char but I can't use it for printing
        let c = byte as char;
        // println!(  "{}", c );
        term::move_cursor( &stdout, 0, term_size.cols - 2 ).unwrap();

        if c.is_control() {
            // all the spaces are to remove any `Char: {}` that was there previously
            write!( stdout, "Binary: {0:08b}  ASCII: {0:#3?}              \r", byte ).unwrap();
        } else if c != 'q' {
            write!( stdout, "Binary: {0:08b}  ASCII: {0:#3?}  Char: {1:#?}\r", byte, c ).unwrap();
        }

        term::move_cursor( &stdout, cursor_x, term_size.cols ).unwrap();

        match byte {
            127 => {
                cursor_x -= 1;
                term::move_cursor( &stdout, cursor_x, term_size.cols ).unwrap();
                write!( stdout,  " " ).unwrap();
                term::move_cursor( &stdout, cursor_x, term_size.cols ).unwrap(); // so that the cursor doesn't lag a box behind
            }
            113 => { // q ==> quit
                writeln!( stdout, "\n\rQuitting... \r" ).unwrap();
                break;
            }
            _ => {
                write!( stdout, "{}", c ).unwrap();
                cursor_x += 1;
            }
        };

        stdout.flush().unwrap();
    };

    term::exit_raw_mode();
    Ok(())
}
