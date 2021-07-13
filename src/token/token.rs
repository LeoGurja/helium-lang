use super::Operator;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  // Special
  Illegal,
  Eof,

  // Identifiers + literals
  Id(String),
  Integer(String),
  String(String),

  // Operators
  Operator(Operator),

  // Delimiters
  Comma,
  Semicolon,

  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  LeftBracket,
  RightBracket,

  // Keywords
  Function,
  Let,
  True,
  False,
  If,
  Else,
  Return,
  For,
  While,
  In,
}

impl std::default::Default for Token {
  fn default() -> Self {
    Token::Eof
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let string;
    write!(
      f,
      "'{}'",
      match self {
        Self::For => "for",
        Self::In => "in",
        Self::While => "while",
        Self::Let => "let",
        Self::If => "if",
        Self::Else => "else",
        Self::Return => "return",
        Self::True => "true",
        Self::False => "false",
        Self::Function => "fn",
        Self::LeftParen => "(",
        Self::LeftBrace => "{",
        Self::LeftBracket => "[",
        Self::RightParen => ")",
        Self::RightBrace => "}",
        Self::RightBracket => "]",
        Self::Comma => ",",
        Self::Eof => "EOF",
        Self::Illegal => "illegal",
        Self::Operator(operator) => {
          string = operator.to_string();
          &string
        }
        Self::Integer(value) | Self::Id(value) | Self::String(value) => value,
        Self::Semicolon => ";",
      }
    )
  }
}

impl Token {
  pub fn lookup_id(id: &str) -> Token {
    match id {
      "fn" => Token::Function,
      "let" => Token::Let,
      "if" => Token::If,
      "else" => Token::Else,
      "return" => Token::Return,
      "true" => Token::True,
      "false" => Token::False,
      "for" => Token::For,
      "while" => Token::While,
      "in" => Token::In,
      _ => Token::Id(id.to_owned()),
    }
  }
}
