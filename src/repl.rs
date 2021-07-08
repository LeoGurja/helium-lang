use crate::lexer::Lexer;
use crate::parser::{Parser, ParserError};
use crate::visitor::Visitor;
use std::io;
use std::io::Write;

pub fn repl() {
  print_welcome();
  let visitor = Visitor::new();
  let mut parser;
  let mut program;
  let mut result;
  let mut lexer;

  loop {
    lexer = Lexer::new(ask_input(">> "));
    parser = Parser::new(lexer);
    program = parser.parse();
    if parser.errors.len() != 0 {
      print_errors(parser.errors);
      continue;
    }
    println!("{:?}", program);
    result = visitor.visit(&program);
    match result {
      Ok(obj) => println!("{}", obj),
      Err(err) => println!("{}", err),
    }
  }
}

fn print_errors(errors: Vec<ParserError>) {
  for error in errors {
    println!("{}", error)
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
