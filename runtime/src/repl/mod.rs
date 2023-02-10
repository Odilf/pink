use once_cell::sync::Lazy;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use termion::{
    color::{Fg, Magenta},
    style::{Bold, Reset},
};

use pink_runtime::Runtime;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const HISTORY_FILE: &str = ".pink-repl-history";

// TODO: Would be nice if this was `const`
static PROMPT: Lazy<String> = Lazy::new(|| format!("{}{}>>{} ", Fg(Magenta), Bold, Reset));

pub fn run(runtime: Runtime, debug: bool) -> Result<()> {
    if debug {
        println!("Debug mode enabled");
        println!("{runtime}");
    }

    print!("Welcome to {}{}pink!{}", Fg(Magenta), Bold, Reset);
    println!(" (v{})", VERSION.unwrap_or("unknown"));

    let mut rl = Editor::<()>::new()?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(&PROMPT);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let expression = match runtime.parse_expression(line.as_str()) {
                    Ok(expression) => expression,
                    Err(err) => {
                        println!("{err}");
                        continue;
                    }
                };

                if !debug {
                    println!("Result: {}", runtime.eval(expression));
                } else {
                    println!("Parsed expression: {expression}");

                    let results = runtime.evaluations(expression);

                    println!("Results: ");
                    for (i, result) in results.iter().enumerate() {
                        println!("  {i}. {result}");
                    }
                }
            }

            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }

            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(HISTORY_FILE)
}
