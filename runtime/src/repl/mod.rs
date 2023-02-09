use once_cell::sync::Lazy;
use pink::engine::Runtime;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use termion::{
    color::{Fg, Magenta},
    style::{Bold, Reset},
};

// TODO: Would be nice if this was `const`
static PROMPT: Lazy<String> = Lazy::new(|| format!("{}{}>>{} ", Fg(Magenta), Bold, Reset));

pub fn run(runtime: Runtime) -> Result<()> {
    // println!("{:?}", runtime.structures.len());

    let mut rl = Editor::<()>::new()?;
    if rl.load_history(".history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(&PROMPT);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let evaluated = match runtime.eval_str(line.as_str()) {
                    Ok(evaluated) => evaluated,
                    Err(err) => {
                        println!("{err}");
                        continue;
                    }
                };

                for (i, result) in evaluated.iter().enumerate() {
                    match i {
                        0 => println!("Main result: {result}\n"),
                        1 => {
                            println!("Other results:");
                            println!("- {result}")
                        }
                        _ => println!("- {result}"),
                    }
                }

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
