use indicatif::ProgressBar;
use anyhow::{ Result };

pub mod arg_parser;
use crate::arg_parser::{ parse_cli_args, read_file };

pub mod color;
use crate::color::Color;

pub fn run() -> Result<()>{
    let args = parse_cli_args();

    let file_content = search_file( args.path );

    check_lines( file_content.to_string(), args.pattern.to_string(), args.raw, args.case_sensitive, args.count )
}

pub fn search_file( path: std::path::PathBuf ) -> String {
    // let file_content = match read_file( args.path ) {
    //     Ok( content ) => content,
    //     Err(_) => {}
    // };

    let file_content = read_file( path ).unwrap();

    file_content
}

pub fn check_lines( content: String, pattern: String, raw: bool, case_sensitive: bool, count: bool ) -> Result<()> {
    let bar = ProgressBar::new( content.lines().count().try_into().unwrap() );
    let pattern_hl = Color::new( 0, 255, 200 );
    let count_text_col = Color::new( 255, 150, 150 );
    let mut found_lines: Vec<String> = Vec::new();
    
    for line in content.lines() {
        let line = match case_sensitive { false => line.to_lowercase(), true => line.to_string() };
        let pattern = match case_sensitive { false => pattern.to_lowercase(), true => pattern.to_string() };

        if line.contains( &pattern ) {
            if raw {
                bar.println( &line );
                found_lines.push( line.to_string() );
            } else {
                let line = &color::pattern_color( line.to_string(), &pattern, &pattern_hl );
                bar.println( line );
                found_lines.push( line.to_string() );
            }
        }
        bar.inc( 1 )
    }

    bar.finish_and_clear();

    if ! raw || count {
        let count_text = format!( "found {} line(s).", found_lines.len() );
        println!( "{}", color::colored( &count_text_col, &count_text ) );
    }

    Ok(())
}
