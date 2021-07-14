use super::Expression;

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
