mod debug;
mod files;
mod fuzzy_find;
mod handle;
mod run;
mod term;

pub const MAIN_SECTION_X: u16 = 5;

fn main() {
    // Code should probably go here
    let result = run::run().unwrap();
    println!("Result: {:?}", result);
}
