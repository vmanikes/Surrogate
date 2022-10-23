use glob::{glob, Paths};
use crate::errors::Error;

#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

// TODO tests
/// Takes in a reference to the path and returns all the full qualified file paths of the .tpl files
pub fn get_tpl_file_paths(path: &String) -> Result<Vec<String>, Error> {
    let mut template_glob_pattern = path.clone();
    template_glob_pattern.push_str("/**/*.tpl");

    let template_files: Paths = match glob(template_glob_pattern.as_str()) {
        Ok(paths) => paths,
        Err(e) => return Err(Error::PatternError),
    };

    let mut results: Vec<String> = Vec::new();

    for entry in template_files {
        let path = match entry {
            Ok(readable_path) => format!("{}", readable_path.display()),
            Err(_) => return Err(Error::GlobDisplayError),
        };

        results.push(path);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_tpl_file_paths() {
    }
}