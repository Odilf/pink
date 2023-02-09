use clap::Parser;
use pink::parser;

mod repl;

fn main() {
    let cli = Cli::parse();
    // let structure = cli.path.map_or(INTRINSIC.clone(), |path| parser::parse_file(path.as_str()).unwrap());
    let structure = parser::parse_file(cli.path.unwrap().into()).unwrap();

    repl::run(structure).unwrap();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: Option<String>,
}
