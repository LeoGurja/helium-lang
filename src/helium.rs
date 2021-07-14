use crate::ast::Statement;
use crate::env::Env;
use crate::error::Error;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;
use crate::visitor::Visitor;
use std::cell::RefCell;
use std::rc::Rc;

pub fn run(input: String) -> Result<Object, Vec<Error>> {
  let visitor = Visitor::new();
  let program = parse(input)?;
  let result = visitor.visit(&program);

  match result {
    Err(err) => Err(vec![err]),
    Ok(ok) => Ok(ok),
  }
}

pub fn import(env: &Rc<RefCell<Env>>, input: String) -> Result<(), Vec<Error>> {
  let visitor = Visitor::from(env.clone());
  let program = parse(input)?;

  match visitor.visit(&program) {
    Ok(..) => Ok(()),
    Err(err) => Err(vec![err]),
  }
}

fn parse(input: String) -> Result<Vec<Statement>, Vec<Error>> {
  let mut parser = Parser::new(Lexer::new(input));
  let program = parser.parse();
  if parser.errors.len() == 0 {
    Ok(program)
  } else {
    Err(parser.errors)
  }
}
