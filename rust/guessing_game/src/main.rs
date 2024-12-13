use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guessing game woo");
    let num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess ( or type quit to quit ):");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line")
        ;
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {num},
            Err(_) => {
                if guess.trim().to_lowercase() == "quit" {
                    println!("bye bye");
                    break;
                } else {
                    println!("pls num :(");
                    continue;
                };
            }
        };
    
        println!("you guessed: {guess}");
        // println!("you guessed: {}", guess);
    
        match guess.cmp(&num) {
            Ordering::Less => println!("too low"),
            Ordering::Greater => println!("too high"),
            Ordering::Equal => {
                println!("you win woo");
                break;
            },
        }
    }

}
