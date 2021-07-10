use super::Expression;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
  Let(String, Expression),
  Return(Option<Expression>),
  Expression(Expression),
  Null,
}

impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Statement::Null => unreachable!(),
      Statement::Let(ident, value) => write!(f, "let {} = {};", ident, value),
      Statement::Return(None) => write!(f, "return;"),
      Statement::Return(Some(exp)) => write!(f, "return {};", exp),
      Statement::Expression(exp) => write!(f, "{};", exp),
    }
  }
}
