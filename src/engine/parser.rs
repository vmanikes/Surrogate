use std::fs;
use crate::errors::Error;
use log::error;
use serde_json::{Value};

/// Reds the Realm.json from the root of the repo and creates an internal JSON representation from it
pub fn realm_json_parser() -> Result<Value, Error>{
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

#[cfg(test)]
mod tests {
    use crate::engine::parser::realm_json_parser;

    #[test]
    fn test_realm_json_parser() {
        realm_json_parser();
    }
}