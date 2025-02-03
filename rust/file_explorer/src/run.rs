use std::{ 
    self,
    io::{ self, Write, stdin, },
};
use crate::term;

pub fn run() -> Result<(), io::Error> {
    // Actual run code goes here

    let stdin = stdin();

    term::enter_raw_mode();

    // term::exit_raw_mode();
    Ok(())
}