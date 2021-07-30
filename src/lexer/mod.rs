#[cfg(test)]
mod test;

pub use logos::Lexer;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a> {
  // Values
  #[regex(r"[a-zA-Z_][a-zA-Z\d_]*")]
  Id(&'a str),
  #[regex(r"\d+", |lex| lex.slice().parse())]
  Integer(i64),
  #[regex(r#""(?:\\.|[^"\\])*""#, remove_quotes)]
  #[regex(r#"'(?:\\.|[^'\\])*'"#, remove_quotes)]
  String(&'a str),

  // Operators
  #[regex(r"(\.|([+\-*/<>!=]=?))")]
  Operator(&'a str),

  // Delimiters
  #[token(",")]
  Comma,
  #[token(";")]
  Semicolon,
  #[token(":")]
  Colon,
  #[token("(")]
  LeftParen,
  #[token(")")]
  RightParen,
  #[token("{")]
  LeftBrace,
  #[token("}")]
  RightBrace,
  #[token("[")]
  LeftBracket,
  #[token("]")]
  RightBracket,

  // Keywords
  #[token("fn")]
  Function,
  #[token("let")]
  Let,
  #[token("true")]
  True,
  #[token("false")]
  False,
  #[token("if")]
  If,
  #[token("else")]
  Else,
  #[token("return")]
  Return,
  #[token("for")]
  For,
  #[token("while")]
  While,
  #[token("in")]
  In,

  // Special
  #[regex(r"[ \n\t\f]+", logos::skip)]
  #[error]
  Illegal,
  Eof,
}

fn remove_quotes<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
  Some(&lex.slice()[1..lex.slice().len() - 1])
}

pub fn lex<'a>(input: &'a str) -> Lexer<'a, Token> {
  Token::lexer(input)
}
