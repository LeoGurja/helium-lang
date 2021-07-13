use crate::ast::Statement;
use crate::env::Env;
use crate::errors::Error;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;
use crate::visitor::Visitor;
use std::cell::RefCell;
use std::rc::Rc;

pub fn run(input: String) -> Result<Object, Error> {
  let parser = Parser::new(Lexer::new(input));
  let visitor = Visitor::new();

  match visitor.visit(&parse(parser)?) {
    Ok(obj) => Ok(obj),
    Err(err) => Err(Error::EvalError(err)),
  }
}

pub fn import(env: &Rc<RefCell<Env>>, input: String) -> Result<(), Error> {
  let parser = Parser::new(Lexer::new(input));
  let visitor = Visitor::from(env.clone());

  match visitor.visit(&parse(parser)?) {
    Ok(..) => Ok(()),
    Err(err) => Err(Error::EvalError(err)),
  }
}

fn parse(mut parser: Parser) -> Result<Vec<Statement>, Error> {
  match parser.parse() {
    Ok(block) => Ok(block),
    Err(errs) => Err(Error::ParserError(errs)),
  }
}
