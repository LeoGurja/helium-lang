use std::fmt;

pub struct Program {
  statements: Vec<Statement>,
}

impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for stmt in &self.statements {
      write!(f, "{}", stmt)?;
    }
    Ok(())
  }
}

pub enum Statement {
  Let(String, Expression),
  Return(Option<Expression>),
  Expression(Expression),
}

impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Statement::Let(ident, value) => write!(f, "let {} = {};", ident, value),
      Statement::Return(None) => write!(f, "return;"),
      Statement::Return(Some(exp)) => write!(f, "return {};", exp),
      Statement::Expression(exp) => write!(f, "{};", exp),
    }
  }
}

pub enum Expression {
  Id(String),
  Integer(i64),
  String(String),
  Boolean(bool),
}

impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Expression::Id(ident) => write!(f, "{}", ident),
      Expression::Integer(value) => write!(f, "{}", value),
      Expression::String(s) => write!(f, "\"{}\"", s),
      Expression::Boolean(value) => write!(f, "{}", value),
    }
  }
}
