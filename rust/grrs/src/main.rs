// use std::{ io::{ self, Write }, env::args, fs };
use std::fs;
use anyhow::{ Context, Result };
use clap::Parser;
use indicatif::ProgressBar;

fn colored(color: &Color, text: &str) -> String {
    return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", color.r, color.g, color.b, text);
}

fn pattern_color( mut text: String, pattern: &str, color: &Color ) -> String {
    let idx: usize = text.to_lowercase().find( pattern ).unwrap().try_into().unwrap();

    let ptrn_len: usize = pattern.len().try_into().unwrap();
    let idx2: usize = idx + ptrn_len;

    let original_pattern_occurrance = &text[ idx..idx2 ];

    text.replace_range(
        idx..idx2,
        &colored( color, original_pattern_occurrance )
    );

    text
}

#[ derive( Debug ) ]
struct Color {
    r: i32,
    g: i32,
    b: i32,
}

/// Search for a pattern in lines of a file, and output lines that contain it.
#[ derive( Parser, Debug ) ]
struct Cli {
    /// Pattern to look for
    pattern: String,

    /// Path to the file to search
    path: std::path::PathBuf,

    /// Search should be case-sensitive
    #[ arg( short, long, default_value_t = false ) ]
    case_sensitive: bool,

    /// Search should only return raw, for piping into later processes
    #[ arg( short, long, default_value_t = false ) ]
    raw: bool,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    let content: String = fs::read_to_string( &args.path )
        .with_context( || format!( "could not read file: `{}`", args.path.display() ) )?;
    
    let progress_bar: ProgressBar = ProgressBar::new( content.lines().count().try_into().unwrap() );
    let mut lines: Vec<String> = vec![];
    let pattern_hl: Color = Color {
        r: 0,
        g: 255,
        b: 200,
    };

    for line in content.lines() {
        if args.case_sensitive {
            if line.contains( &args.pattern ) {
                // println!( "{}", line )
                lines.push( line.to_string() );

                if ! args.raw {
                    let line = &pattern_color( line.to_string(), &args.pattern, &pattern_hl );
                    progress_bar.println( line );
                } else {
                    progress_bar.println( line );
                };

            };
        } else {
            if line.to_lowercase().contains( &args.pattern.to_lowercase() ) {
                // println!( "{}", line )
                lines.push( line.to_string() );

                if !args.raw {
                    let line = &pattern_color( line.to_string(), &args.pattern, &pattern_hl );
                    progress_bar.println( line );
                } else {
                    progress_bar.println( line );
                };

            };
        };
        progress_bar.inc( 1 );
    }
    progress_bar.finish_and_clear();

    let out_text_color: Color = Color {
        r: 150,
        g: 200,
        b: 255,
    };

    if ! args.raw {
        let out_text = format!( "found {} line(s).", lines.len() );
        println!( "{}", colored( &out_text_color, &out_text ) );
    };

    Ok(())
}
