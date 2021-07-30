use crate::{ast::Statement, lexer::Lexer, object::Object, parser::Parser};

#[test]
fn integer_object() {}

fn parse(input: &str) -> Vec<Statement> {
  let lexer = Lexer::new(input.to_owned());
  let parser = Parser::new(lexer);
  let program = parser.parse();

  if parser.errors.len() != 0 {
    for err in parser.errors {
      println!("{}", err);
    }
    assert!(false);
  }
  return parser.parse();
}
