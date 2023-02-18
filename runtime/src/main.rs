use std::{path::PathBuf, collections::BTreeMap};

use clap::Parser;
use pink_runtime::{parse_file, Structure, Runtime};

mod repl;

fn main() {
    let cli = Cli::parse();

    // TODO: uggo
    let runtime = match cli.path {
        Some(path) => match parse_file(path) {
            Ok(structure) => structure,
            Err(err) => {
                eprintln!("Error while parsing file: {}", err);
                std::process::exit(1);
            }
        }
        None => Runtime::new(BTreeMap::from([("instrinsic".to_owned(), Structure::intrinsic())])),
    };
        
    match repl::run(runtime, cli.debug) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("Error while running REPL: {}", err);
            std::process::exit(1);
        }
    };
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: Option<PathBuf>,

    #[clap(short, long, default_value_t = false)]
    debug: bool,
}
