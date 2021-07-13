use crate::ast::Expression;
use crate::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ParserError {
  UnexpectedToken(Token, Token),
  ExpectedExpression(Token),
  ExpectedPrefix(Token),
  ParsingError(String, String),
  ExpectedId(Expression),
}

impl fmt::Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::ExpectedId(got) => {
        write!(f, "ExpectedId:\n\tExpected an id, got {} instead", got)
      }
      Self::UnexpectedToken(expected, got) => {
        write!(
          f,
          "UnexpectedToken:\n\tExpected {}, got {:?} instead",
          expected, got
        )
      }
      Self::ParsingError(expected, got) => {
        write!(
          f,
          "ParsingError:\n\tExpected a valid {}, got {} instead",
          expected, got
        )
      }
      Self::ExpectedExpression(got) => {
        write!(
          f,
          "ExpectedExpression:\n\tExpected an expression, got {:?} instead",
          got
        )
      }
      Self::ExpectedPrefix(got) => {
        write!(
          f,
          "ExpectedPrefix:\n\tExpected a prefix, got {:?} instead",
          got
        )
      }
    }
  }
}
