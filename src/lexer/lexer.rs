use super::token::Token;
use std::cell::{Cell, RefCell};
use std::iter::Peekable;
use std::mem;
use std::str::Chars;

pub struct Lexer {
  input: String,
  position: Cell<usize>,
  current: Cell<char>,
  chars: RefCell<Peekable<Chars<'static>>>,
}

impl Lexer {
  pub fn new(input: String) -> Self {
    let chars = RefCell::new(unsafe { mem::transmute(input.chars().peekable()) });
    let lexer = Lexer {
      input,
      position: Cell::new(0),
      current: Cell::new('\0'),
      chars,
    };
    lexer.advance();
    lexer
  }

  pub fn next_token(&self) -> Token {
    self.skip_whitespace();
    let current = self.current.get();

    let token = match current {
      '=' => {
        if self.peek() == '=' {
          self.advance();
          Token::Equals
        } else {
          Token::Assign
        }
      }
      '"' | '\'' => self.collect_string(),
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
        if current.is_alphabetic() || current == '_' {
          return self.collect_id();
        } else if current.is_numeric() {
          return self.collect_number();
        } else {
          Token::Illegal
        }
      }
    };
    self.advance();
    token
  }

  fn collect_string(&self) -> Token {
    let opening_quote = self.current.get();
    self.advance();
    let start_position = self.position.get();
    let mut is_escaped = false;
    let mut current = self.current.get();
    while current != opening_quote || is_escaped {
      is_escaped = current == '\\';
      self.advance();
      current = self.current.get();
      if current == '\0' {
        return Token::Illegal;
      }
    }

    let end_position = self.position.get();

    Token::String(
      self.input[start_position..end_position]
        .replace(
          &format!("\\{}", opening_quote),
          &format!("{}", opening_quote),
        )
        .to_owned(),
    )
  }

  fn collect_id(&self) -> Token {
    let start_position = self.position.get();
    let mut current = self.current.get();
    while current.is_alphabetic() || current == '_' {
      self.advance();
      current = self.current.get();
    }

    Token::lookup_id(&self.input[start_position..self.position.get()])
  }

  fn collect_number(&self) -> Token {
    let start_position = self.position.get();
    let mut current = self.current.get();
    while current.is_numeric() {
      self.advance();
      current = self.current.get();
    }

    Token::Integer(self.input[start_position..self.position.get()].to_owned())
  }

  fn advance(&self) {
    let current = self.current.get();
    self.position.set(
      self.position.get()
        + if current == '\0' {
          0
        } else {
          current.len_utf8()
        },
    );
    self
      .current
      .set(self.chars.borrow_mut().next().unwrap_or('\0'));
  }

  fn skip_whitespace(&self) {
    while self.current.get().is_whitespace() {
      self.advance();
    }
  }

  fn peek(&self) -> char {
    self.chars.borrow_mut().peek().cloned().unwrap_or('\0')
  }
}
