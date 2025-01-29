use std::io;
use clap::Parser;

#[ derive( Parser, Debug ) ]
struct Cli {
    /// Whether to print in debug mode
    #[ arg( long, default_value_t=false ) ]
    debug: bool,
}

fn main() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut to_binary = String::new();
    let args: Cli = Cli::parse();

    io::stdin()
        .read_line(&mut to_binary)
        .expect("Failed to read line")
    ;

    for char in to_binary.trim().to_lowercase().chars() {
        if char == ' ' {
            print!( "{:08b}", 32 );
            if args.debug {
                println!( ": {}", char );
            }
        } else {
            print!( "{:08b}", &alphabet.find( char ).unwrap() + 64 );
            if args.debug {
                println!( ": {}", char );
            }
        }
    };

    println!("")
}
