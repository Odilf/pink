use std::path::PathBuf;

use clap::Parser;
use pink::parser;

mod repl;

fn main() {
    let cli = Cli::parse();
    
    let path = cli.path.unwrap();
    dbg!(path.as_path());
    let structure = parser::parse_file(path).unwrap();

    repl::run(structure).unwrap();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: Option<PathBuf>,
}
