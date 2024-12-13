use clap::{ App, Arg };
use rand::Rng;

fn main() {
    let matches = App::new("random")
        .version("1.0")
        .arg(
            Arg::with_name("max")
                .short('m')
                .long("max")
                .takes_value(true)
                .help("maximum value")
        )
        .get_matches();

    let max: i32 = matches
        .value_of("max")
        .unwrap_or("100")
        .parse()
        .expect("'max' must be a number!");

    let random_number = rand::thread_rng().gen_range(1..=max);
    println!("{random_number}")
}
