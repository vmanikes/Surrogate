#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Pattern,
    GlobDisplay,
    NoMatchingTemplateFiles,
    UnableToFetchCurrentDir,
    NoSurrogateJSONFile,
    UnableToReadJSON,
    UnableToCreateFile,
    RenderingError
}
