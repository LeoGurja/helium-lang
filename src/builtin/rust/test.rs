use crate::lexer::Token;
use crate::{ast::Statement, object::Object, parser::Parser, visitor::Visitor};
use logos::Logos;

#[test]
fn array_push() {
  let input = "push([1,2,3,4], 5)";
  assert_eq!(
    visit(input),
    Object::Array(vec![
      Object::Integer(1),
      Object::Integer(2),
      Object::Integer(3),
      Object::Integer(4),
      Object::Integer(5)
    ])
  )
}

#[test]
fn string_push() {
  let input = "push('leonardo', ' gurgel')";
  assert_eq!(visit(input), Object::String("leonardo gurgel".to_owned()))
}

#[test]
fn array_len() {
  let input = "len([1,2,3,4])";
  assert_eq!(visit(input), Object::Integer(4))
}

#[test]
fn array_first() {
  let input = "first([1,2,3,4])";
  assert_eq!(visit(input), Object::Integer(1))
}

#[test]
fn array_last() {
  let input = "last([1,2,3,4,5])";
  assert_eq!(visit(input), Object::Integer(5))
}

#[test]
fn string_len() {
  let input = "len('leonardo gurgel')";
  assert_eq!(visit(input), Object::Integer(15))
}

#[test]
fn string_first() {
  let input = "first('leonardo gurgel')";
  assert_eq!(visit(input), Object::String("l".to_owned()))
}

#[test]
fn empty_array_first() {
  let input = "first([])";
  assert_eq!(visit(input), Object::Null)
}

#[test]
fn empty_string_first() {
  let input = "first('')";
  assert_eq!(visit(input), Object::Null)
}

#[test]
fn string_last() {
  let input = "last('leonardo')";
  assert_eq!(visit(input), Object::String("o".to_owned()))
}

#[test]
fn empty_string_last() {
  let input = "last('')";
  assert_eq!(visit(input), Object::Null)
}

#[test]
fn empty_array_last() {
  let input = "last([])";
  assert_eq!(visit(input), Object::Null)
}

#[test]
fn print() {
  let input = "print('hello world')";
  assert_eq!(visit(input), Object::Null)
}

fn visit(input: &str) -> Object {
  let visitor = Visitor::new();
  let program = parse(Parser::new(Token::lexer(input)));
  visitor.visit(&program).unwrap()
}

fn parse(mut parser: Parser) -> Vec<Statement> {
  let program = parser.parse();

  for err in parser.errors {
    println!("{}", err);
  }
  program
}
