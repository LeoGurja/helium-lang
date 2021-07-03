mod test;
mod token;

use std::iter::Peekable;
use std::mem;
use std::str::Chars;
pub use token::Token;

pub struct Lexer {
  input: String,
  position: usize,
  current: char,
  chars: Peekable<Chars<'static>>,
}

impl Lexer {
  pub fn new(input: String) -> Self {
    let chars = unsafe { mem::transmute(input.chars().peekable()) };
    let mut lexer = Lexer {
      input,
      position: 0,
      current: '\0',
      chars,
    };
    lexer.advance();
    lexer
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();

    let token = match self.current {
      '=' => {
        if self.peek() == '=' {
          self.advance();
          Token::Equals
        } else {
          Token::Assign
        }
      }
      ';' => Token::Semicolon,
      '(' => Token::LeftParen,
      ')' => Token::RightParen,
      '[' => Token::LeftBracket,
      ']' => Token::RightBracket,
      '{' => Token::LeftBrace,
      '}' => Token::RightBrace,
      ',' => Token::Comma,
      '+' => Token::Plus,
      '-' => Token::Minus,
      '/' => Token::Slash,
      '*' => Token::Asterisk,
      '!' => {
        if self.peek() == '=' {
          self.advance();
          Token::NotEquals
        } else {
          Token::Bang
        }
      }
      '<' => Token::LessThan,
      '>' => Token::GreaterThan,
      '\0' => Token::Eof,
      _ => {
        if self.current.is_alphabetic() || self.current == '_' {
          return self.collect_id();
        } else if self.current.is_numeric() {
          return self.collect_number();
        } else {
          Token::Illegal
        }
      }
    };
    self.advance();
    token
  }

  fn collect_id(&mut self) -> Token {
    let start_position = self.position;

    while self.current.is_alphabetic() || self.current == '_' {
      self.advance();
    }

    Token::lookup_id(&self.input[start_position..self.position])
  }

  fn collect_number(&mut self) -> Token {
    let start_position = self.position;

    while self.current.is_numeric() {
      self.advance();
    }

    Token::Integer(self.input[start_position..self.position].to_owned())
  }

  fn advance(&mut self) {
    self.position += if self.current == '\0' {
      0
    } else {
      self.current.len_utf8()
    };
    self.current = self.chars.next().unwrap_or('\0');
  }

  fn skip_whitespace(&mut self) {
    while self.current.is_whitespace() {
      self.advance();
    }
  }

  fn peek(&mut self) -> char {
    self.chars.peek().cloned().unwrap_or('\0')
  }
}
