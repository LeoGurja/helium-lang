use super::Operator;

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
  Colon,

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
