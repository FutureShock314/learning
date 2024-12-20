use anyhow::Result;
use grrs::run;

fn main() -> Result<()> {
    // let args: Cli = Cli::parse();

    // let content: String = fs::read_to_string( &args.path )
    //     .with_context( || format!( "could not read file: `{}`", args.path.display() ) )?;
    
    // let progress_bar: ProgressBar = ProgressBar::new( content.lines().count().try_into().unwrap() );
    // let mut lines: Vec<String> = vec![];
    // let pattern_hl: Color = Color::new( 0, 255, 200 );

    // for line in content.lines() {
    //     if args.case_sensitive {
    //         if line.contains( &args.pattern ) {
    //             // println!( "{}", line )
    //             lines.push( line.to_string() );

    //             if ! args.raw {
    //                 let line = &pattern_color( line.to_string(), &args.pattern, &pattern_hl );
    //                 progress_bar.println( line );
    //             } else {
    //                 progress_bar.println( line );
    //             };

    //         };
    //     } else {
    //         if line.to_lowercase().contains( &args.pattern.to_lowercase() ) {
    //             // println!( "{}", line )
    //             lines.push( line.to_string() );

    //             if !args.raw {
    //                 let line = &pattern_color( line.to_string(), &args.pattern, &pattern_hl );
    //                 progress_bar.println( line );
    //             } else {
    //                 progress_bar.println( line );
    //             };

    //         };
    //     };
    //     progress_bar.inc( 1 );
    // }
    // progress_bar.finish_and_clear();

    // let out_text_color: Color = Color::new( 150, 200, 255 );

    // if ( ! args.raw ) || args.count {
    //     let out_text = format!( "found {} line(s).", lines.len() );
    //     println!( "{}", colored( &out_text_color, &out_text ) );
    // };

    // Ok(())

    run()
}
