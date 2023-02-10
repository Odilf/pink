use once_cell::sync::Lazy;
use pink::engine::Runtime;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

use termion::{
    color::{Fg, Magenta},
    style::{Bold, Reset},
};

// TODO: Would be nice if this was `const`
static PROMPT: Lazy<String> = Lazy::new(|| format!("{}{}>>{} ", Fg(Magenta), Bold, Reset));

pub fn run(runtime: Runtime) -> Result<()> {
    println!("Welcome to {}{}Pink!{} (v{})\n", Fg(Magenta), Bold, Reset, VERSION.unwrap_or("unknown"));
    println!("{runtime}");

    let mut rl = Editor::<()>::new()?;
    if rl.load_history(".history.txt").is_err() {
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

                let result = runtime.eval(expression);
                println!("Main result: {result}\n");

                // Flush
                println!();
            }

            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Ending session");
                break;
            }

            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(".history.txt")
}
