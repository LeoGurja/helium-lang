use super::block::Block;
use super::operators::{Infix, Prefix};
use crate::helpers::comma_separated;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
  Id(String),
  Integer(i64),
  String(String),
  Boolean(bool),
  Call(String, Vec<Expression>),
  Prefix(Prefix, Box<Expression>),
  Infix(Infix, Box<Expression>, Box<Expression>),
  If(Box<Expression>, Block, Option<Block>),
  Function(Option<String>, Vec<String>, Block),
}

impl Expression {
  pub const TRUE: Expression = Expression::Boolean(true);
  pub const FALSE: Expression = Expression::Boolean(false);
}

impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Expression::Call(function, arguments) => {
        write!(f, "{}({})", function, comma_separated(arguments))
      }
      Expression::Function(name, args, ..) => write!(
        f,
        "{}({})",
        name.clone().unwrap_or_else(|| String::from("anonymous")),
        comma_separated(args)
      ),
      Expression::Id(ident) => write!(f, "{}", ident),
      Expression::Integer(value) => write!(f, "{}", value),
      Expression::String(s) => write!(f, "\"{}\"", s),
      Expression::Boolean(value) => write!(f, "{}", value),
      Expression::Prefix(operator, exp) => write!(f, "({}{})", operator, exp),
      Expression::Infix(operator, left, right) => {
        write!(f, "({} {} {})", left, operator, right)
      }
      Expression::If(condition, ..) => {
        write!(f, "if({})", condition)
      }
    }
  }
}
