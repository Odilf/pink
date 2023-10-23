#[cfg(feature = "repl")]
use std::{collections::BTreeMap, path::PathBuf};

#[cfg(feature = "repl")]
use clap::Parser;
#[cfg(feature = "repl")]
use pink_runtime::{parse_file, Runtime, Structure};

#[cfg(feature = "repl")]
mod repl;

#[cfg(not(feature = "repl"))]
fn main() {
    eprintln!("This binary was compiled without the REPL feature");
    std::process::exit(1);
}

#[cfg(feature = "repl")]
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
        },

        None => Runtime::new(BTreeMap::from([(
            "instrinsic".to_owned(),
            Structure::intrinsic(),
        )])),
    };

    match repl::run(runtime, cli.debug) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("Error while running REPL: {}", err);
            std::process::exit(1);
        }
    };
}

#[cfg(feature = "repl")]
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: Option<PathBuf>,

    #[clap(short, long, default_value_t = false)]
    debug: bool,
}
