use super::Statement;
use crate::token::Operator;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
  Id(String),
  Integer(i64),
  String(String),
  Boolean(bool),
  Call(Box<Expression>, Vec<Expression>),
  Prefix(Operator, Box<Expression>),
  Infix(Operator, Box<Expression>, Box<Expression>),
  If(Box<Expression>, Box<Statement>, Option<Box<Statement>>),
  Function(Option<String>, Vec<String>, Box<Statement>),
  Array(Vec<Expression>),
  Hash(Vec<(Expression, Expression)>),
  Index(Box<Expression>, Box<Expression>),
}

impl Expression {
  pub const TRUE: Expression = Expression::Boolean(true);
  pub const FALSE: Expression = Expression::Boolean(false);
}
