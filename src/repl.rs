use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::visitor::Visitor;
use std::io;
use std::io::Write;

pub fn repl() {
  print_welcome();

  let visitor = Visitor::new();
  loop {
    let input = read();
    let program = Parser::new(Lexer::new(input)).parse();

    match program {
      Ok(program) => match visitor.visit(&program) {
        Ok(obj) => println!("{}", obj),
        Err(err) => println!("{}", err),
      },
      Err(errs) => println!("Parse errors:\n\t{:?}", errs),
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
  let mut stdout = io::stdout();
  let stdin = io::stdin();
  let mut input = String::new();

  print!("{}", ">> ");
  stdout.flush().expect("Failed to flush stdout");
  stdin
    .read_line(&mut input)
    .expect("Failed to read line from stdin");

  input
}
