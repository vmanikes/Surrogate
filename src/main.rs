mod engine;
mod errors;
mod logo;

use clap::Parser;

// TODO Build in release mode
// TODO Build for different platforms

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct RealmArgs {
    #[arg(
        short,
        long,
        long_help = "path where realm should run",
        required = true
    )]
    directory: String, // TODO Default vault is current dir
}

use handlebars::Handlebars;
use serde_json::json;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    logo::generate::print_logo();

    // let args = RealmArgs::parse();

    let mut reg = Handlebars::new();
    reg.set_strict_mode(true);

    reg.register_template_string("tpl_1", "Good afternoon, {{name.first}}").unwrap();

    match reg.render("tpl_1", &json!({"name": "dsdsd"})) {
        Ok(val) => println!("{}", val),
        Err(e) => println!("{}", e),
    };

    // register template using given name
    // .unwrap();
    // println!("{}", reg.render("tpl_1", &json!({"name": "foo"})).unwrap());
}
