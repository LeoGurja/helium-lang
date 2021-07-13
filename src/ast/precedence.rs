use crate::token::Operator;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Precedence {
  Lowest,
  Assign,      // =
  Equals,      // == | !=
  LessGreater, // >  | <
  Sum,         // +  | -
  Product,     // *
  Prefix,      // -X or !X
  Call,        // myFunction(X)
  Index,       // array[index]
}

impl Precedence {
  pub fn from(token: &Operator) -> Self {
    match token {
      Operator::Assign => Precedence::Assign,
      Operator::Equals | Operator::NotEquals => Precedence::Equals,
      Operator::GreaterThan | Operator::LessThan => Precedence::LessGreater,
      Operator::Plus | Operator::Minus => Precedence::Sum,
      Operator::Slash | Operator::Asterisk => Precedence::Product,
      _ => Precedence::Lowest,
    }
  }
}
