use super::Statement;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
  IdAssignment(String, Box<Expression>),
  IndexAssignment(Box<Expression>, Box<Expression>, Box<Expression>),
  Id(String),
  Integer(i64),
  String(String),
  Call(Box<Expression>, Vec<Expression>),
  Prefix(String, Box<Expression>),
  Infix(String, Box<Expression>, Box<Expression>),
  Conditional(Box<Expression>, Box<Statement>, Option<Box<Statement>>),
  Function(Option<String>, Vec<String>, Rc<Statement>),
  Array(Vec<Expression>),
  Hash(Vec<(Expression, Expression)>),
  Index(Box<Expression>, Box<Expression>),
  True,
  False,
  Null,
}
