use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;
use crate::visitor::{EvalError, Visitor};

#[test]
fn array_len() {
  let input = String::from("len([1,2,3,4])");
  assert_eq!(visit(input), Object::Integer(4))
}

#[test]
fn array_first() {
  let input = String::from("first([1,2,3,4])");
  assert_eq!(visit(input), Object::Integer(1))
}

#[test]
fn array_last() {
  let input = String::from("last([1,2,3,4,5])");
  assert_eq!(visit(input), Object::Integer(5))
}

#[test]
fn string_len() {
  let input = String::from("len('leonardo gurgel')");
  assert_eq!(visit(input), Object::Integer(15))
}

#[test]
fn string_first() {
  let input = String::from("first('leonardo gurgel')");
  assert_eq!(visit(input), Object::String(String::from("l")))
}

#[test]
fn string_last() {
  let input = String::from("last('leonardo')");
  assert_eq!(visit(input), Object::String(String::from("o")))
}

fn visit(input: String) -> Object {
  let visitor = Visitor::new();
  let result = visitor.visit(&Parser::new(Lexer::new(input)).parse());

  match result {
    Err(err) => panic!("visitor returned error: {}", err),
    Ok(value) => value,
  }
}
