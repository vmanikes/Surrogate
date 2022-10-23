#[derive(Debug, PartialEq)]
pub enum Error {
    PatternError,
    GlobDisplayError,
    NoMatchingTemplateFiles,
}
