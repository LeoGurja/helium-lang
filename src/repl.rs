use crate::lexer::{Lexer, Token};
use std::io;
use std::io::Write;

pub fn repl() {
  print_welcome();
  loop {
    let mut lexer = Lexer::new(ask_input(">> "));

    let mut token = lexer.next_token();
    while token != Token::Eof {
      println!("{:?}", token);
      token = lexer.next_token();
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
