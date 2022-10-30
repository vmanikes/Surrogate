use crate::engine::template;
use crate::errors::Error;
use crate::errors::Error::{RenderingError, UnableToCreateFile};
use handlebars::Handlebars;
use log::{error, info};
use serde_json::Value;
use std::fs;

/// Reads the Surrogate.json from the root of the repo and creates an internal JSON representation from it
fn surrogate_json_parser() -> Result<Value, Error> {
    let current_directory = match std::env::current_dir() {
        Ok(dir) => format!("{}", dir.display()),
        Err(e) => {
            error!("{}: {}", Error::UnableToFetchCurrentDir.to_string(), e);
            return Err(Error::UnableToFetchCurrentDir);
        }
    };

    let surrogate_file_contents =
        match fs::read_to_string(format!("{}/Surrogate.json", current_directory)) {
            Err(_) => {
                error!("make sure you have Surrogate.json in the root of your repo");
                return Err(Error::NoSurrogateJSONFile);
            }
            Ok(contents) => contents,
        };

    let v: Value = match serde_json::from_str(surrogate_file_contents.as_str()) {
        Err(e) => {
            error!("unable to parse the surrogate json: {}", e);
            return Err(Error::UnableToParseJSON);
        }
        Ok(val) => val,
    };

    Ok(v)
}

/// Reads all the .tpl files, injects the values from surrogate json file and
/// writes an actual file by removing the .tpl suffix
pub fn generate_files_from_templates(path: &str) -> Result<(), Error> {
    let templates: Vec<String> = template::get_tpl_file_paths(path)?;
    let surrogate_file_contents: Value = surrogate_json_parser()?;

    let mut handlebar_registry = Handlebars::new();
    handlebar_registry.set_strict_mode(true);

    info!("entering the magical world of Surrogate, brace for impact");
    info!("found a total of {} templates in {}", templates.len(), path);

    for (idx, template) in templates.iter().enumerate() {
        info!("parsing template: {}", template);

        handlebar_registry
            .register_template_file(idx.to_string().as_str(), &template)
            .unwrap();

        let parsed_file_path = match template.strip_suffix(".tpl") {
            Some(path) => path,
            None => template.as_str(),
        };

        let mut output_file = match fs::File::create(parsed_file_path) {
            Ok(file) => file,
            Err(err) => {
                error!("unable to create file {}: {}", parsed_file_path, err);
                return Err(UnableToCreateFile);
            }
        };

        match handlebar_registry.render_to_write(
            idx.to_string().as_str(),
            &surrogate_file_contents,
            &mut output_file,
        ) {
            Err(err) => {
                error!("unable to render template: {}", err);
                return Err(RenderingError);
            }
            Ok(_) => continue,
        }
    }

    info!("done parsing all the template");
    info!("exiting the magical world of Surrogate");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::engine::parser::{generate_files_from_templates, surrogate_json_parser};
    use crate::errors::Error;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_surrogate_file_parser() {
        let val = surrogate_json_parser().unwrap();
        assert_eq!(
            val["file"],
            "this file is for surrogate testing, DO NOT DELETE"
        )
    }

    #[test]
    fn test_generate_files_from_templates() {
        struct TestCase {
            test_name: String,
            path: String,
            expected_result: Result<(), Error>,
        }

        let test_cases: [TestCase; 2] = [
            TestCase {
                test_name: "valid test template path is current dir ( with . )".to_string(),
                path: ".".to_string(),
                expected_result: Ok(()),
            },
            TestCase {
                test_name: "path without any tpl files".to_string(),
                path: "/tmp".to_string(),
                expected_result: Err(Error::GlobDisplay),
            },
        ];

        for test_case in test_cases {
            println!("Test Case: {}", test_case.test_name);

            let result = generate_files_from_templates(test_case.path.as_str());
            assert_eq!(test_case.expected_result, result);
        }
    }
}
