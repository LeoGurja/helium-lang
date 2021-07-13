use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Operator {
  Assign,
  Plus,
  Minus,
  Bang,
  Asterisk,
  Slash,
  LessThan,
  GreaterThan,
  Equals,
  NotEquals,
}

impl fmt::Display for Operator {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Assign => "=",
        Self::Asterisk => "*",
        Self::Bang => "!",
        Self::GreaterThan => ">",
        Self::LessThan => "<",
        Self::Equals => "==",
        Self::Slash => "/",
        Self::Plus => "+",
        Self::Minus => "-",
        Self::NotEquals => "!=",
      }
    )
  }
}
