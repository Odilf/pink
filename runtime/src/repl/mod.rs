use once_cell::sync::Lazy;
use pink::engine::Structure;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use termion::{color::{Fg, Magenta}, style::{Bold, Reset}};

// TODO: Would be nice if this was `const`
static PROMPT: Lazy<String> = Lazy::new(|| format!("{}{}>>{} ", Fg(Magenta), Bold, Reset));

pub fn run(structure: Structure) -> Result<()> {
    println!("{structure}");

    let mut rl = Editor::<()>::new()?;
    if rl.load_history(".history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(&PROMPT);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

				let evaluated = match structure.eval_str(line.as_str()) {
					Ok(evaluated) => evaluated,
					Err(err) => {
						println!("{err}");
						continue;
					}
				};

				print!("Result: {evaluated}");

				// Flush
				println!();
            }

            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Ending session");
                break
            }

            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    };

    rl.save_history(".history.txt")
}