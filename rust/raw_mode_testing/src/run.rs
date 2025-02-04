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

    let mut cursor_x = 2;
    let cols_usize = term_size.cols as usize;

    write!( stdout, "╭{:─<1$}\n", "─", cols_usize - 1 ).unwrap();
    write!( stdout, "│\n" ).unwrap();
    write!( stdout, "╰{:─<1$}", "─", cols_usize - 1 ).unwrap();
    
    stdout.flush().unwrap();

    term::enter_raw_mode();

    for byte in stdin.bytes() {
        let byte = byte.unwrap(); // would use char but I can't use it for printing
        let c = byte as char;
        // println!(  "{}", c );

        debug::check_byte( byte, c, 2, term_size.rows - 3 );

        let cursor_y = term_size.rows - 2;

        term::move_cursor( cursor_x, cursor_y ).unwrap();

        match byte {
            127 => {
                // if cursor_x > 0 {
                //     cursor_x -= 1;
                //     term::move_cursor( cursor_x, cursor_y ).unwrap();
                //     write!( stdout,  " " ).unwrap();
                //     // so that the cursor doesn't lag a box behind
                //     term::move_cursor( cursor_x, cursor_y ).unwrap();
                // }

                cursor_x = handle::on_backspace( cursor_x, cursor_y, 2 );
            }
            113 /* q */
            | 27 /* escape */
            | 3 /* <C-c> */
            => { // q ==> quit
                writeln!( stdout, "\n\n\rQuitting... \r" ).unwrap();
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
