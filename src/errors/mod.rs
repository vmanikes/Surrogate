#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Pattern,
    GlobDisplay,
    NoMatchingTemplateFiles,
}
