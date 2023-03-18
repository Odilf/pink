use std::collections::BTreeSet;
use std::time::Instant;

use once_cell::sync::Lazy;

use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{Editor, Result};

use termion::{
    color::{Fg, Magenta},
    style::{Bold, Reset},
};

use pink_runtime::{Expression, Runtime};

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

    let mut rl = Editor::<(), FileHistory>::new()?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(&PROMPT);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();

                let expression = match runtime.parse_expression(line.as_str()) {
                    Ok(expression) => expression,
                    Err(err) => {
                        println!("{err}");
                        continue;
                    }
                };

                let _evaluations = runtime.evaluations(expression, &mut repl_loop_callback());
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

fn repl_loop_callback() -> impl FnMut(&BTreeSet<Expression>) {
    let time_start = Instant::now();

    let mut smallest_expression: Option<Expression> = None;

    move |evaluations: &BTreeSet<Expression>| {
        if smallest_expression.is_none() {
            smallest_expression = Some(evaluations.iter().next().unwrap().clone());
        }

        let candidate = evaluations.iter().next().unwrap();

        if candidate < &smallest_expression.clone().unwrap() {
            smallest_expression = Some(candidate.clone());
            println!(
                "Found smaller expression: {} (took {:.2}s)",
                candidate,
                (Instant::now() - time_start).as_secs_f32()
            );
        }
    }
}
