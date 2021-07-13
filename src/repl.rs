use crate::helium;
use std::io;
use std::io::Write;

pub fn repl() {
  print_welcome();

  loop {
    let result = helium::run(ask_input(">> "));

    match result {
      Ok(obj) => println!("{}", obj),
      Err(err) => println!("{}", err),
    }
  }
}

fn print_welcome() {
  println!(
    "+-------------------+
| 2                 |
|    _    _         |
|   | |  | |        |
|   | |__| | ___    |
|   |  __  |/ _ \\   |
|   | |  | |  __/   |
|   |_|  |_|\\___|   |
|                   |
|       4.003       |
+-------------------+"
  );
  println!("Welcome to the Helium repl!");
  println!("Feel free to type in commands");
}

fn ask_input(prompt: &str) -> String {
  let mut stdout = io::stdout();
  let stdin = io::stdin();
  let mut input = String::new();

  print!("{}", prompt);
  stdout.flush().expect("Failed to flush stdout");
  stdin
    .read_line(&mut input)
    .expect("Failed to read line from stdin");

  input
}
