use crate::env::Env;
use crate::error::Error;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;
use crate::visitor::Visitor;
use std::cell::RefCell;
use std::rc::Rc;

pub fn run(input: String) -> Result<Object, Vec<Error>> {
  let mut parser = Parser::new(Lexer::new(input));
  let visitor = Visitor::new();
  let program = parser.parse()?;
  match visitor.visit(&program) {
    Err(err) => Err(vec![err]),
    Ok(ok) => Ok(ok),
  }
}

pub fn import(env: &Rc<RefCell<Env>>, input: String) -> Result<(), Vec<Error>> {
  let mut parser = Parser::new(Lexer::new(input));
  let visitor = Visitor::from(env.clone());
  let program = parser.parse()?;
  match visitor.visit(&program) {
    Err(err) => Err(vec![err]),
    Ok(..) => Ok(()),
  }
}
