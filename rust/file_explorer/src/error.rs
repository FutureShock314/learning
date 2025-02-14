use crossterm::style::{
    Color, SetBackgroundColor,
    Print, 
};
use crossterm::execute;
use std::io::Stdout;

#[ derive( Debug ) ]
pub enum FeError {
    Permission
}

impl std::error::Error for FeError {}

impl std::fmt::Display for FeError {
    fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        match self {
            FeError::Permission => write!( f, " Permission Denied " ),
        }
    }
}

pub fn error_print( mut screen: &Stdout, err: FeError ) {
    execute!(
        screen,
        SetBackgroundColor( Color::Red ),
        Print( format!( "{}", err ) ),
    ).unwrap();
}
