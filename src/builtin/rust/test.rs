use crate::ast::Statement;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;
use crate::visitor::Visitor;

#[test]
fn array_push() {
  let input = String::from("push([1,2,3,4], 5)");
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
  let input = String::from("push('leonardo', ' gurgel')");
  assert_eq!(
    visit(input),
    Object::String(String::from("leonardo gurgel"))
  )
}

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

#[test]
fn print() {
  let input = String::from("print('hello world')");
  assert_eq!(visit(input), Object::Null)
}

fn visit(input: String) -> Object {
  let visitor = Visitor::new();
  let result = visitor.visit(&parse(Parser::new(Lexer::new(input))));

  match result {
    Err(err) => panic!("visitor returned error: {}", err),
    Ok(value) => value,
  }
}

fn parse(mut parser: Parser) -> Vec<Statement> {
  let program = parser.parse();
  match program {
    Ok(block) => block,
    Err(errs) => panic!("Parser has errors:\n\t{:?}", errs),
  }
}
