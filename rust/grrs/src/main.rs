// use std::{ io::{ self, Write }, env::args, fs };
use std::fs;
use anyhow::{ Context, Result };
use clap::Parser;
use indicatif::ProgressBar;

/// Search for a pattern in lines of a file, and output lines that contain it.
#[derive(Parser)]
struct Cli {
    /// Pattern to look for
    pattern: String,

    /// Path to the file to search
    path: std::path::PathBuf,

    /// Case-sensitive
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

    // println!("pattern: {:?}\npath: {:?}", args.pattern, args.path); // debug line
    // println!( "{}", out.join(", ") );
    println!( "{:?}", out );
    for line in &out {
        let index = line.to_lowercase().find( &args.pattern );

        line.replace_range( index..Some(args.pattern.len()), "" );

        println!( "{}", line );
    };

    println!( "found {} lines.", out.len() );

    Ok(())
}
