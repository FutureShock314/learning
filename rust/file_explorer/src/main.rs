mod debug;
mod fuzzy_find;
mod handle;
mod run;
mod term;

fn main() {
    // Code should probably go here
    let result = run::run().unwrap();
    println!("Result: {:?}", result);
}
