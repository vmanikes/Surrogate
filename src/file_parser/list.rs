use std::fs;

// TODO How to generate documentation
pub fn list_template_files(directory: String) {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}
