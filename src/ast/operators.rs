use crate::lexer::Token;
use crate::parser::ParserError;
use std::fmt;

type Result<T> = std::result::Result<T, ParserError>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Precedence {
  Lowest,
  Equals,      // ==
  LessGreater, // > or <
  Sum,         // +
  Product,     // *
  Prefix,      // -X or !X
  Call,        // myFunction(X)
  Index,       // array[index]
}

#[derive(Debug, PartialEq, Clone)]
pub enum Prefix {
  Bang,
  Minus,
}

impl fmt::Display for Prefix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Prefix::Bang => write!(f, "!"),
      Prefix::Minus => write!(f, "-"),
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Infix {
  Equals,
  NotEquals,
  LessThan,
  GreaterThan,
  Plus,
  Minus,
  Asterisk,
  Slash,
}

impl fmt::Display for Infix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Infix::Equals => write!(f, "=="),
      Infix::NotEquals => write!(f, "!="),
      Infix::LessThan => write!(f, "<"),
      Infix::GreaterThan => write!(f, ">"),
      Infix::Plus => write!(f, "+"),
      Infix::Minus => write!(f, "-"),
      Infix::Asterisk => write!(f, "*"),
      Infix::Slash => write!(f, "/"),
    }
  }
}

impl Prefix {
  pub fn from(token: &Token) -> Result<Prefix> {
    match token {
      Token::Bang => Ok(Prefix::Bang),
      Token::Minus => Ok(Prefix::Minus),
      _ => Err(ParserError::ExpectedPrefix(token.clone())),
    }
  }
}

impl Infix {
  pub fn from(token: &Token) -> (Precedence, Option<Infix>) {
    match token {
      Token::Equals => (Precedence::Equals, Some(Infix::Equals)),
      Token::NotEquals => (Precedence::Equals, Some(Infix::NotEquals)),
      Token::LessThan => (Precedence::LessGreater, Some(Infix::LessThan)),
      Token::GreaterThan => (Precedence::LessGreater, Some(Infix::GreaterThan)),
      Token::Plus => (Precedence::Sum, Some(Infix::Plus)),
      Token::Minus => (Precedence::Sum, Some(Infix::Minus)),
      Token::Slash => (Precedence::Product, Some(Infix::Slash)),
      Token::Asterisk => (Precedence::Product, Some(Infix::Asterisk)),
      Token::LeftParen => (Precedence::Call, None),
      Token::LeftBracket => (Precedence::Index, None),
      _ => (Precedence::Lowest, None),
    }
  }
}
