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

fn main() {
    logo::generate::print_logo();

    // let args = RealmArgs::parse();
}
