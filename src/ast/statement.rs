use super::Expression;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
  Block(Vec<Statement>),
  Let(String, Expression),
  Return(Option<Expression>),
  Expression(Expression),
  While(Expression, Box<Statement>),
  For(String, Expression, Box<Statement>),
  Null,
}

impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Statement::Block(block) => {
        write!(f, "{{ {:?} }}", block)
      }
      Statement::For(variable, iterable, block) => {
        write!(f, "for {} in {} do {:?}", variable, iterable, block)
      }
      Statement::While(condition, block) => write!(f, "while {} do {:?}", condition, block),
      Statement::Null => unreachable!(),
      Statement::Let(ident, value) => write!(f, "let {} = {};", ident, value),
      Statement::Return(None) => write!(f, "return;"),
      Statement::Return(Some(exp)) => write!(f, "return {};", exp),
      Statement::Expression(exp) => write!(f, "{};", exp),
    }
  }
}
