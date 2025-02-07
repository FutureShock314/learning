mod debug;
mod files;
mod fuzzy_find;
mod handle;
mod run;
mod term;

pub const MAIN_SECTION_X: u16 = 5;
pub const MAIN_SECTION_FRAC: f64 = 0.35;

fn main() {
    // Code should probably go here
    let result = run::run().unwrap();
    println!("Result: {:?}", result);
}
