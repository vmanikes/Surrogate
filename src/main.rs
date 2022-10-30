mod engine;
mod errors;
mod logo;

use clap::Parser;
use log::{error};
use crate::errors::Error;

// TODO Build for different platforms

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
#[clap(version)]
struct SurrogateArgs {
    #[clap(default_value_t='.'.to_string(),short, long)]
    /// Directory against which surrogate should run
    pub directory: String,
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    logo::print_logo();

    let args = SurrogateArgs::parse();

    match engine::parser::generate_files_from_templates(args.directory.as_str()) {
        Err(e) => {
            error!("{:?}", e)
        }
        _ => {}
    }
}
