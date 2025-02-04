use std::{
    self,
    io::{
        self,
        Read, Write,
        stdin, stdout
    },
};
use crate::term::{ self, TermSize };
use crate::input;

pub fn run() -> Result<(), io::Error> {
    let stdin = stdin();
    let mut stdout = stdout();
    let term_size: TermSize = term::get_term_size()?;
    // println!( "Terminal size: {:?}", term_size );
    println!( "" );

    let paths = std::fs::read_dir("./").unwrap();
    for path in paths {
        println!( "{}", path.unwrap().path().display() )
    }

    let mut cursor_x = 0;

    write!( stdout, "{:-<1$}\n", "-", term_size.cols as usize ).unwrap();
    write!( stdout, "\n" ).unwrap();
    write!( stdout, "{:-<1$}", "-", term_size.cols as usize ).unwrap();
    
    stdout.flush().unwrap();

    term::enter_raw_mode();

    for byte in stdin.bytes() {
        let byte = byte.unwrap(); // would use char but I can't use it for printing
        let c = byte as char;
        // println!(  "{}", c );
        term::move_cursor( 2, term_size.rows - 3 ).unwrap();

        if c.is_control() {
            // all the spaces are to remove any `Char: {}` that was there previously
            write!( stdout, "Binary: {0:08b}  ASCII: {0:#3?}              \r", byte ).unwrap();
        } else if c != 'q' {
            write!( stdout, "Binary: {0:08b}  ASCII: {0:#3?}  Char: {1:#?}\r", byte, c ).unwrap();
        }

        let cursor_y = term_size.rows - 2;

        term::move_cursor( cursor_x, cursor_y ).unwrap();

        match byte {
            127 => {
                if cursor_x > 0 {
                    cursor_x -= 1;
                    term::move_cursor( cursor_x, cursor_y ).unwrap();
                    write!( stdout,  " " ).unwrap();
                    // so that the cursor doesn't lag a box behind
                    term::move_cursor( cursor_x, cursor_y ).unwrap();
                }
            }
            113 /* q */
            | 27 /* escape */
            | 3 /* <C-c> */
            => { // q ==> quit
                writeln!( stdout, "\n\rQuitting... \r" ).unwrap();
                break;
            }
            _ => {
                write!( stdout, "{}", c ).unwrap();
                cursor_x += 1;
            }
        }

        stdout.flush().unwrap();
    }

    term::move_cursor( 0, term_size.rows ).unwrap();
    term::exit_raw_mode();
    Ok(())
}
