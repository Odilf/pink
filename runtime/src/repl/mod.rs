use pink::engine::Structure;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};


pub fn run(structure: Structure) -> Result<()> {
	// `()` can be used when no completer is required
    let mut rl = Editor::<()>::new()?;
    // if rl.load_history("history.txt").is_err() {
    //     println!("No previous history.");
    // }
    loop {
        let readline = rl.readline(">> ");
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

				print!("{evaluated}");

				// Flush
				println!("");
            }

            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            }

            Err(ReadlineError::Eof) => {
                println!("Ending session");
                break
            }

            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    };

	Ok(())
}