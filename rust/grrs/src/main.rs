// use std::{ io::{ self, Write }, env::args, fs };
use std::fs;
use anyhow::{ Context, Result };
use clap::Parser;
use indicatif::ProgressBar;

fn colored(r: i32, g: i32, b: i32, text: &str) -> String {
    return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text);
}

/// Search for a pattern in lines of a file, and output lines that contain it.
#[derive(Parser)]
struct Cli {
    /// Pattern to look for
    pattern: String,

    /// Path to the file to search
    path: std::path::PathBuf,

    /// Search should be case-sensitive
    #[ arg( short, default_value_t = false ) ]
    case_sensitive: bool,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    let content: String = fs::read_to_string( &args.path )
        .with_context( || format!( "could not read file: `{}`", args.path.display() ) )?;
    
    let progress_bar: ProgressBar = ProgressBar::new( content.lines().count().try_into().unwrap() );
    let mut out: Vec<String> = vec![];

    for line in content.lines() {
        if args.case_sensitive {
            if line.contains( &args.pattern ) {
                // println!( "{}", line )
                out.push( line.to_string() )
            }
        } else {
            if line.to_lowercase().contains( &args.pattern ) {
                // println!( "{}", line )
                out.push( line.to_string() )
            }
        };
        progress_bar.inc( 1 );
    }
    progress_bar.finish();

    // println!("pattern: {:?}\npath: {:?}", args.pattern, args.path); // debug
    // println!( "{}", out.join(", ") ); // alt output
    // println!( "{:?}", out ); // debug
    for line in &mut out {
        let idx: usize = line.to_lowercase().find( &args.pattern ).unwrap().try_into().unwrap();
        // let idx_usize: u64 = idx.try_into().unwrap();

        let ptrn_len: usize = args.pattern.len().try_into().unwrap();
        let idx2: usize = idx + ptrn_len;

        let original_pattern_occurrance = &line[ idx..idx2 ];

        // line.replace_range( idx..idx2, &colored( 0, 255, 200, "    omg guys it's here    " ) ); // debug test
        line.replace_range( idx..idx2, &colored( 0, 255, 200, original_pattern_occurrance ) );

        // println!( "{}", idx ); // debug
        println!( "{}", line );
    };

    let out_text = format!( "found {} lines.", out.len() );
    println!( "{}", colored( 150, 200, 255, &out_text ) );

    Ok(())
}
