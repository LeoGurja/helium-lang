use std::{convert::From, rc::Rc};

#[derive(Debug, PartialEq)]
pub enum Statement {
  Block(Vec<Statement>),
  VariableDeclaration(String, Expression),
  Return(Expression),
  Expression(Expression),
  WhileLoop(Expression, Box<Statement>),
  ForLoop(String, Expression, Box<Statement>),
  Null,
}

impl Statement {
  pub fn while_loop(condition: Expression, block: Self) -> Self {
    Self::WhileLoop(condition, Box::new(block))
  }

  pub fn for_loop(string: &str, iterable: Expression, block: Self) -> Self {
    Self::ForLoop(string.to_owned(), iterable, Box::new(block))
  }
}

impl From<Expression> for Statement {
  fn from(exp: Expression) -> Self {
    Self::Expression(exp)
  }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
  Id(String),
  Integer(i64),
  String(String),
  Boolean(bool),
  Call(Box<Expression>, Vec<Expression>),
  Prefix(String, Box<Expression>),
  Infix(String, Box<Expression>, Box<Expression>),
  Conditional(Box<Expression>, Box<Statement>, Option<Box<Statement>>),
  Function(Option<String>, Vec<String>, Rc<Statement>),
  Array(Vec<Expression>),
  Hash(Vec<(Expression, Expression)>),
  Index(Box<Expression>, Box<Expression>),
  Null,
}

impl Expression {
  pub const TRUE: Self = Self::Boolean(true);
  pub const FALSE: Self = Self::Boolean(false);

  pub fn call(id: Expression, args: Vec<Expression>) -> Self {
    Self::Call(Box::new(id), args)
  }

  pub fn conditional(
    condition: Expression,
    consequence: Statement,
    alternative: Option<Statement>,
  ) -> Self {
    Self::Conditional(
      Box::new(condition),
      Box::new(consequence),
      alternative.and_then(|alt| Some(Box::new(alt))),
    )
  }

  pub fn infix(operator: &str, left: Expression, right: Expression) -> Self {
    Self::Infix(operator.to_owned(), Box::new(left), Box::new(right))
  }

  pub fn prefix(operator: &str, expression: Expression) -> Self {
    Self::Prefix(operator.to_owned(), Box::new(expression))
  }

  pub fn index(indexed: Expression, index: Expression) -> Self {
    Self::Index(Box::new(indexed), Box::new(index))
  }

  pub fn function(name: Option<&str>, args: Vec<String>, block: Statement) -> Self {
    Self::Function(name.and_then(|n| Some(n.to_owned())), args, Rc::new(block))
  }
}
