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
