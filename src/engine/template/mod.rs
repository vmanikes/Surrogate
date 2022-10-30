use crate::errors::Error;
use glob::{glob, Paths};
use log::error;

/// Takes in a reference to the path and returns all the full qualified file paths of the .tpl files
pub fn get_tpl_file_paths(path: &str) -> Result<Vec<String>, Error> {
    let template_files: Paths = match glob(format!("{}/**/*.tpl", path).as_str()) {
        Ok(paths) => paths,
        Err(err) => {
            error!("{}: {}", Error::Pattern.to_string(), err);
            return Err(Error::Pattern);
        }
    };

    let mut results: Vec<String> = Vec::new();

    for entry in template_files {
        let path = match entry {
            Ok(readable_path) => format!("{}", readable_path.display()),
            Err(err) => {
                error!("{}: {}", Error::GlobDisplay.to_string(), err);
                return Err(Error::GlobDisplay);
            }
        };

        results.push(path);
    }

    if results.is_empty() {
        return Err(Error::NoMatchingTemplateFiles);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use crate::engine::template::get_tpl_file_paths;
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

            let template_paths = get_tpl_file_paths(test_case.path.as_str());
            match template_paths {
                Ok(val) => assert_eq!(val.len(), test_case.expected_template_count),
                Err(e) => assert_eq!(e, test_case.expected_error.unwrap()),
            }
        }
    }
}
