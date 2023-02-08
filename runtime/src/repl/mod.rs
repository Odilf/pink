use std::io::{stdin, stdout, Write, self};
use pink::{engine::{Structure, eval}, parser::expression};

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

				for token in evaluated {
					print!("{} ", token);
				}

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
    }
    rl.save_history("history.txt")

}

// pub fn run(structure: Structure) {
// 	let stdin = stdin();
// 	let mut stdout = stdout().into_raw_mode().unwrap();
	
//     write!(stdout, "Welcome to pink. \n\rPress `ctrl+D` to exit. \n\r").unwrap();
//     write!(stdout, "> ").unwrap();
// 	stdout.flush().unwrap();

// 	let mut buffer = String::new();
//     for key in stdin.keys() {
// 		match key.unwrap() {
// 			Key::Ctrl('d') => break,

// 			Key::Char('\n') => {
// 				print!("\n\r");
// 				for token in structure.eval_str(buffer.as_str()).unwrap() {
// 					print!("{} ", token);
// 				}
				
// 				// Start new buffer
// 				print!("\n\r> ");
// 				buffer = String::new();
// 			}

// 			Key::Backspace => {
// 				buffer.pop();
// 				print!("{}", termion::clear::CurrentLine);
// 				print!("\r> ");
// 				for c in buffer.chars() {
// 					print!("{}", c);
// 				}
// 			}

// 			Key::Char(c) => {
// 				buffer.push(c);
// 				print!("{}", c);
// 			}

// 			_ => {}
// 		}
// 		stdout.flush().unwrap();
// 	}

// 		// match line {
// 		// 	Ok(line) => {
// 		// 		println!("You typed: {}", line);
// 		// 	},
// 		// 	Err(_) => {
// 		// 		println!("Error reading line");
// 		// 	}
// 		// }
//         // match c.unwrap() {
//         //     Key::Ctrl('d') => break,
//         //     Key::Char('q') => break,
//         //     Key::Char(c) => print!("{}", c),
//         //     _ => {}
//         // }
//         stdout.flush().unwrap();
//     // }

//     // write!(stdout, "{}", termion::cursor::Show).unwrap();
// }