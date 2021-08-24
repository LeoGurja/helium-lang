#[cfg(test)]
mod test;

pub use logos::Lexer;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'source> {
  // Values
  #[regex(r"[a-zA-Z_][a-zA-Z\d_]*")]
  Id(&'source str),
  #[regex(r"\d+", |lex| lex.slice().parse())]
  Integer(i64),
  #[regex(r#""(?:\\.|[^"\\])*""#, collect_string)]
  #[regex(r#"'(?:\\.|[^'\\])*'"#, collect_string)]
  String(String),

  // Operators
  #[regex(r"[+\-/*]?=")]
  Assign(&'source str),
  #[regex(r"([+\-/*]|[<>!=]=?)")]
  Operator(&'source str),

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

pub fn lexer<'source>(source: &'source str) -> Lexer<'source, Token<'source>> {
  Token::lexer(source)
}

fn collect_string<'source>(lex: &mut Lexer<'source, Token<'source>>) -> Option<String> {
  let mut string = "".to_owned();
  let mut escape_next_character = false;
  for character in lex.slice()[1..lex.slice().len() - 1].chars() {
    if escape_next_character {
      println!("escaping: {}", character);
      match character {
        c @ '\\' | c @ '\'' | c @ '"' => string.push(c),
        'n' => string.push('\n'),
        't' => string.push('\t'),
        'r' => string.push('\r'),
        '0' => string.push('\0'),
        _ => return None,
      }
      escape_next_character = false;
      continue;
    }
    if character == '\\' {
      escape_next_character = true;
    } else {
      escape_next_character = false;
      string.push(character);
    }
  }
  Some(string)
}
