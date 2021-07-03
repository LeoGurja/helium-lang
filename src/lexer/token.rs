#[derive(Debug, PartialEq)]
pub enum Token {
  // Special
  Illegal,
  Eof,

  // Identifiers + literals
  Id(String),
  Integer(String),

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
