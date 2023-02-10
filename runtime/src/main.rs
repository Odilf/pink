use std::path::PathBuf;

use clap::Parser;
use pink_runtime::parse_file;

mod repl;

fn main() {
    let cli = Cli::parse();

    let path = cli.path.unwrap();
    let structure = parse_file(path).unwrap();

    repl::run(structure, cli.debug).unwrap();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: Option<PathBuf>,

    #[clap(short, long, default_value_t = false)]
    debug: bool,
}
