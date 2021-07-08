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
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "'{}'",
      match self {
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
        Self::Assign => "=",
        Self::Asterisk => "*",
        Self::Bang => "!",
        Self::Comma => ",",
        Self::Eof => "EOF",
        Self::GreaterThan => ">",
        Self::LessThan => "<",
        Self::Equals => "==",
        Self::Slash => "/",
        Self::Plus => "+",
        Self::Minus => "-",
        Self::Illegal => "illegal",
        Self::Integer(value) | Self::Id(value) | Self::String(value) => value,
        Self::NotEquals => "!=",
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
      _ => Token::Id(id.to_owned()),
    }
  }
}
