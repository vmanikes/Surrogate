use clap::Parser;

// TODO Build in release mode
// TODO Build for different platforms
// TODO Add github actions

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct RealmArgs {
    #[arg(short = 'f', long, long_help = "path to realm.json file, defaults to cwd", required = false)]
    realm_file_path: String, // TODO Default vault is current dir
    #[arg(short, long, long_help = "path where realm should run", required = true)]
    directory: String, // TODO Default vault is current dir
}

fn main() {
    let args = RealmArgs::parse();
}
