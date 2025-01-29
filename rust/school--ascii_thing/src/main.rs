use std::io;

fn main() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut to_binary = String::new();

    io::stdin()
        .read_line(&mut to_binary)
        .expect("Failed to read line")
    ;

    for char in to_binary.trim().to_lowercase().chars() {
        print!( "{:08b}", &alphabet.find( char ).unwrap() + 64 );
    };

    println!("")
}
