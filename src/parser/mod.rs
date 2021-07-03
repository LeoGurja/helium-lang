mod ast;
#[cfg(test)]
mod test;

use crate::lexer::{Lexer, Token};
use ast::Program;
use std::mem;

struct Parser {
  lexer: Lexer,
  current: Token,
  peek: Token,
}

impl Parser {
  pub fn new(input: String) -> Self {
    let mut lexer = Lexer::new(input);
    Parser {
      current: lexer.next_token(),
      peek: lexer.next_token(),
      lexer,
    }
  }

  pub fn parse() {}

  fn advance(&mut self) {
    self.current = mem::replace(&mut self.peek, self.lexer.next_token());
  }
}
