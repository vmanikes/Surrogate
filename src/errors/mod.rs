use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Pattern,
    GlobDisplay,
    NoMatchingTemplateFiles,
    UnableToFetchCurrentDir,
    NoSurrogateJSONFile,
    UnableToParseJSON,
    UnableToCreateFile,
    RenderingError
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Pattern => write!(f, "pattern error"),
            Error::GlobDisplay => write!(f, "glob display error"),
            Error::NoMatchingTemplateFiles => write!(f, "no matching template files"),
            Error::UnableToFetchCurrentDir => write!(f, "unable to list current directory"),
            Error::NoSurrogateJSONFile => write!(f, "no surrogate json file"),
            Error::UnableToParseJSON => write!(f, "unable to parse JSON"),
            Error::UnableToCreateFile => write!(f, "unable to create file"),
            Error::RenderingError => write!(f, "surrogate rendering error"),
        }
    }
}