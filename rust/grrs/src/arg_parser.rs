use clap::Parser;
use anyhow::{ Context, Result };

/// Search for a pattern in lines of a file, and output lines that contain it.
#[ derive( Parser, Debug ) ]
pub struct Cli {
    /// Pattern to look for
    pub pattern: String,

    /// Path to the file to search
    pub path: std::path::PathBuf,

    /// Search should be case-sensitive
    #[ arg( short, long, default_value_t = false ) ]
    pub case_sensitive: bool,

    /// Search should only return raw lines, for piping into later processes
    #[ arg( short, long, default_value_t = false ) ]
    pub raw: bool,

    /// Show line count at end of output ( overrides `raw` )
    #[ arg( short = 'C', long, default_value_t = false ) ]
    pub count: bool,
}

pub fn parse_cli_args() -> Cli {
    let args: Cli = Cli::parse();

    args
}

pub fn read_file( path: std::path::PathBuf ) -> Result<String> {
    let file_content = std::fs::read_to_string( &path )
        .with_context( || format!( "Failed to read file `{}`", path.display() ) )?;

    Ok( file_content )
}
