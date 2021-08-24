use crate::{
  ast::Statement, env::Env, error::Error, lexer::lexer, object::Object, parser::Parser,
  visitor::Visitor,
};
use std::fs;

pub fn run(input: &str) -> Result<Object, Vec<Error>> {
  let visitor = Visitor::new();
  let program = parse(input)?;
  let result = visitor.visit(&program);

  match result {
    Err(err) => Err(vec![err]),
    Ok(ok) => Ok(ok),
  }
}

pub fn import(env: &Env, filename: &str) -> Result<(), Vec<Error>> {
  let file = fs::read_to_string(filename).unwrap();
  let visitor = Visitor::from(env.clone());
  let program = parse(&file)?;

  match visitor.visit(&program) {
    Ok(..) => Ok(()),
    Err(err) => Err(vec![err]),
  }
}

fn parse(input: &str) -> Result<Vec<Statement>, Vec<Error>> {
  let mut parser = Parser::new(lexer(input));
  let program = parser.parse();
  if parser.errors.len() == 0 {
    Ok(program)
  } else {
    Err(parser.errors)
  }
}
