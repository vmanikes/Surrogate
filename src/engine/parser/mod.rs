use std::fs;
use handlebars::Handlebars;
use crate::errors::Error;
use crate::engine::template;
use log::{error, info};
use serde_json::{Value};
use crate::errors::Error::{RenderingError, UnableToCreateFile};

/// Reads the Realm.json from the root of the repo and creates an internal JSON representation from it
fn realm_json_parser() -> Result<Value, Error>{
    let current_directory = match std::env::current_dir() {
        Ok(dir) => format!("{}", dir.display()),
        Err(e) => {
            error!("{}", e);
            return Err(Error::UnableToFetchCurrentDir);
        }
    };

    let realm_file_contents = match fs::read_to_string(format!("{}/Realm.json", current_directory)) {
        Err(e) => {
            error!("{} make sure you have Realm.json in the root of your repo", e);
            return Err(Error::NoRealmJSONFile);
        },
        Ok(contents) => contents
    };

    let v: Value = match serde_json::from_str(realm_file_contents.as_str()) {
        Err(e) => {
            error!("unable to parse the realm json: {}", e);
            return Err(Error::UnableToReadJSON);
        },
        Ok(val) => val,
    };

    Ok(v)
}

/// Reads all the .tpl files, injects the values from realm json file and
/// writes an actual file by removing the .tpl suffix
pub fn generate_files_from_templates(path: &str) -> Result<(), Error> {
    let templates: Vec<String> = template::get_tpl_file_paths(path)?;
    let realm_file_contents: Value = realm_json_parser()?; // TODO can this be passed to handlebar

    let mut handlebar_registry = Handlebars::new();
    handlebar_registry.set_strict_mode(true);

    info!("entering the magical world of REALM, brace for impact");
    info!("found a total of {} templates in {}", templates.len(), path);

    for (idx, template) in templates.iter().enumerate() {
        info!("parsing template: {}", template);

        handlebar_registry.register_template_file(idx.to_string().as_str(), &template).unwrap();

        let parsed_file_path = match template.strip_suffix(".tpl") {
            Some(path) => path,
            None => template.as_str()
        };

        let mut output_file = match fs::File::create(parsed_file_path) {
            Ok(file) => file,
            Err(err) => {
                error!("unable to create file {}: {}", parsed_file_path, err);
                return Err(UnableToCreateFile)
            }
        };

        match handlebar_registry.render_to_write(idx.to_string().as_str(), &realm_file_contents, &mut output_file) {
            Err(err) => {
                error!("unable to render template: {}", err);
                return Err(RenderingError)
            },
            Ok(_) => continue
        }
    }

    info!("done parsing all the template");
    info!("exiting the magical world of Realm");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::engine::parser::{realm_file_parser};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_realm_file_parser() {
        let val = realm_file_parser().unwrap();
        assert_eq!(val["file"], "this file is for realm testing, DO NOT DELETE")
    }
}