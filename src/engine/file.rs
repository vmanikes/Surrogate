use crate::errors::Error;
use glob::{glob, Paths};

/// Takes in a reference to the path and returns all the full qualified file paths of the .tpl files
pub fn get_tpl_file_paths(path: &String) -> Result<Vec<String>, Error> {
    let mut template_glob_pattern = path.clone();
    template_glob_pattern.push_str("/**/*.tpl");

    let template_files: Paths = match glob(template_glob_pattern.as_str()) {
        Ok(paths) => paths,
        Err(_) => return Err(Error::PatternError),
    };

    let mut results: Vec<String> = Vec::new();

    for entry in template_files {
        let path = match entry {
            Ok(readable_path) => format!("{}", readable_path.display()),
            Err(_) => return Err(Error::GlobDisplayError),
        };

        results.push(path);
    }

    if results.len() == 0 {
        return Err(Error::NoMatchingTemplateFiles);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use crate::engine::file::get_tpl_file_paths;
    use crate::errors::Error;
    use pretty_assertions::assert_eq;
    use std::env;

    #[test]
    fn test_get_tpl_file_paths() {
        struct TestCase {
            test_name: String,
            path: String,
            expected_template_count: usize,
            expected_error: Option<Error>,
        }

        let test_cases: [TestCase; 3] = [
            TestCase {
                test_name: "valid test template path is current dir ( with . )".to_string(),
                path: ".".to_string(),
                expected_template_count: 3,
                expected_error: None,
            },
            TestCase {
                test_name: "valid test template path is current dir ( with cwd )".to_string(),
                path: format!("{}", env::current_dir().unwrap().display()),
                expected_template_count: 3,
                expected_error: None,
            },
            TestCase {
                test_name: "no template files exist".to_string(),
                path: format!("{}/src", env::current_dir().unwrap().display()),
                expected_template_count: 0,
                expected_error: Option::from(Error::NoMatchingTemplateFiles),
            },
        ];

        for test_case in test_cases {
            println!("Test Case: {}", test_case.test_name);

            let template_paths = get_tpl_file_paths(&test_case.path);
            match template_paths {
                Ok(val) => assert_eq!(val.len(), test_case.expected_template_count),
                Err(e) => assert_eq!(e, test_case.expected_error.unwrap()),
            }
        }
    }
}
