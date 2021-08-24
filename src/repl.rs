use crate::{lexer::lexer, parser::Parser, visitor::Visitor};
use std::io::{stdin, stdout, Write};

pub fn repl() {
  print_welcome();
  let visitor = Visitor::new();
  loop {
    let input = read();
    let mut parser = Parser::new(lexer(&input));
    let program = parser.parse();

    if parser.errors.len() == 0 {
      match visitor.visit(&program) {
        Ok(obj) => println!("{}", obj),
        Err(err) => println!("{}", err),
      }
    } else {
      for err in parser.errors {
        println!("{}", err)
      }
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

fn read() -> String {
  let mut stdout = stdout();
  let stdin = stdin();
  let mut input = String::new();

  print!("{}", ">> ");
  stdout.flush().expect("Failed to flush stdout");
  stdin
    .read_line(&mut input)
    .expect("Failed to read line from stdin");

  input
}
