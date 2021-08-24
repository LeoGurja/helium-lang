use super::Expression;

#[derive(Debug, PartialEq, Clone)]
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
