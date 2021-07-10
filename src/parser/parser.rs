use super::parser_error::ParserError;
use crate::ast::{Block, Expression, Infix, Precedence, Prefix, Statement};
use crate::lexer::{Lexer, Token};
use std::cell::{Cell, RefCell};

type Result<T> = std::result::Result<T, ParserError>;

pub struct Parser {
  lexer: Lexer,
  peek: Cell<Token>,
  pub errors: RefCell<Vec<ParserError>>,
}

impl Parser {
  pub fn new(lexer: Lexer) -> Self {
    Parser {
      peek: Cell::new(lexer.next_token()),
      lexer,
      errors: RefCell::new(vec![]),
    }
  }

  pub fn parse(&self) -> Block {
    let mut block = vec![];

    loop {
      match self.parse_statement() {
        Ok(Statement::Null) => break,
        Ok(statement) => {
          block.push(statement);
        }
        Err(err) => {
          self.errors.borrow_mut().push(err);
        }
      }
    }

    block
  }

  fn parse_block(&self) -> Result<Block> {
    let mut block = vec![];

    self.eat(Token::LeftBrace)?;
    loop {
      match self.parse_statement() {
        Ok(Statement::Null) => break,
        Ok(statement) => block.push(statement),
        Err(err) => self.errors.borrow_mut().push(err),
      }

      if self.next_if(Token::RightBrace) {
        break;
      }
    }
    Ok(block)
  }

  fn parse_statement(&self) -> Result<Statement> {
    let statement = match self.peek() {
      Token::Let => self.parse_variable_declaration(),
      Token::Return => self.parse_return(),
      Token::Eof | Token::RightBrace => Ok(Statement::Null),
      _ => Ok(Statement::Expression(
        self.parse_expression(Precedence::Lowest)?,
      )),
    };
    self.skip_semicolons();
    statement
  }

  fn parse_expression(&self, precedence: Precedence) -> Result<Expression> {
    let current = self.next();
    let mut left = match current {
      Token::Id(value) => self.parse_id(value),
      Token::Integer(value) => self.parse_integer(value),
      Token::String(value) => Ok(Expression::String(value)),
      Token::True => Ok(Expression::TRUE),
      Token::False => Ok(Expression::FALSE),
      Token::Bang | Token::Minus => self.parse_prefix_expression(current),
      Token::LeftParen => self.parse_grouped_expression(),
      Token::If => self.parse_if_expression(),
      Token::LeftBracket => Ok(Expression::Array(
        self.parse_expression_list(Token::RightBracket)?,
      )),
      // Token::LeftBrace => Some(Parser::parse_hash_literal),
      Token::Function => self.parse_function(),
      _ => Err(ParserError::ExpectedExpression(current)),
    }?;

    let mut operator = Infix::from(self.peek());
    while precedence < operator.0 {
      self.next();
      left = match operator.1 {
        Some(Infix::Index) => self.parse_index_expression(left)?,
        Some(infix) => self.parse_infix_expression(infix, left, operator.0)?,
        _ => break,
      };
      operator = Infix::from(self.peek());
    }
    Ok(left)
  }

  fn parse_infix_expression(
    &self,
    infix: Infix,
    left: Expression,
    precedence: Precedence,
  ) -> Result<Expression> {
    Ok(Expression::Infix(
      infix,
      Box::new(left),
      Box::new(self.parse_expression(precedence)?),
    ))
  }

  fn parse_index_expression(&self, left: Expression) -> Result<Expression> {
    let right = self.parse_expression(Precedence::Lowest)?;
    self.eat(Token::RightBracket)?;
    Ok(Expression::Infix(
      Infix::Index,
      Box::new(left),
      Box::new(right),
    ))
  }

  fn parse_function(&self) -> Result<Expression> {
    let mut args = vec![];

    let name = if let Token::Id(id) = self.peek() {
      self.next();
      Some(id.clone())
    } else {
      None
    };
    self.eat(Token::LeftParen)?;
    while !self.next_if(Token::RightParen) && !self.next_if(Token::Eof) {
      let current = self.next();
      if let Token::Id(arg) = current {
        args.push(arg)
      } else {
        return Err(ParserError::UnexpectedToken(
          Token::Id(String::from("...")),
          current,
        ));
      }
      if !self.next_if(Token::Comma) {
        break;
      }
    }
    let block = self.parse_block()?;

    Ok(Expression::Function(name, args, block))
  }

  fn parse_expression_list(&self, end: Token) -> std::result::Result<Vec<Expression>, ParserError> {
    let mut args = vec![];
    loop {
      args.push(self.parse_expression(Precedence::Lowest)?);
      if !self.next_if(Token::Comma) {
        break;
      }
    }

    let current = self.next();
    if current == end {
      Ok(args)
    } else {
      Err(ParserError::UnexpectedToken(end, current))
    }
  }

  fn parse_if_expression(&self) -> Result<Expression> {
    let condition = self.parse_expression(Precedence::Lowest)?;
    let consequence = self.parse_block()?;
    let alternative = if self.next_if(Token::Else) {
      let else_block = self.parse_block()?;
      Some(else_block)
    } else {
      None
    };

    Ok(Expression::If(
      Box::new(condition),
      consequence,
      alternative,
    ))
  }

  fn parse_grouped_expression(&self) -> Result<Expression> {
    let left = self.parse_expression(Precedence::Lowest);
    self.eat(Token::RightParen)?;
    left
  }

  fn parse_id(&self, id: String) -> Result<Expression> {
    if self.next_if(Token::LeftParen) {
      return self.parse_function_call(id);
    }
    Ok(Expression::Id(id))
  }

  fn parse_function_call(&self, name: String) -> Result<Expression> {
    Ok(Expression::Call(
      name,
      self.parse_expression_list(Token::RightParen)?,
    ))
  }

  fn parse_integer(&self, integer: String) -> Result<Expression> {
    match integer.parse() {
      Ok(value) => Ok(Expression::Integer(value)),
      Err(_) => Err(ParserError::ParsingError(String::from("integer"), integer)),
    }
  }

  fn parse_prefix_expression(&self, prefix_token: Token) -> Result<Expression> {
    let prefix = Prefix::from(prefix_token)?;
    let expression = self.parse_expression(Precedence::Prefix)?;

    Ok(Expression::Prefix(prefix, Box::new(expression)))
  }

  fn parse_variable_declaration(&self) -> Result<Statement> {
    self.next(); // jump let
    let name;
    let current = self.next();
    if let Token::Id(id) = current {
      name = id;
    } else {
      return Err(ParserError::UnexpectedToken(
        Token::Id(String::from("...")),
        current,
      ));
    }

    self.eat(Token::Assign)?;

    let value = self.parse_expression(Precedence::Lowest)?;

    Ok(Statement::Let(name, value))
  }

  fn parse_return(&self) -> Result<Statement> {
    self.next(); // jump return
    let value = self.parse_expression(Precedence::Lowest)?;

    Ok(Statement::Return(Some(value)))
  }

  fn skip_semicolons(&self) {
    let mut current = self.peek();
    while current == Token::Semicolon {
      self.next();
      current = self.peek();
    }
  }

  fn eat(&self, should_be: Token) -> Result<()> {
    let current = self.next();
    if current == should_be {
      Ok(())
    } else {
      Err(ParserError::UnexpectedToken(should_be, current))
    }
  }

  fn next_if(&self, token: Token) -> bool {
    let result = self.peek() == token;
    if result {
      self.next();
    }

    result
  }

  fn next(&self) -> Token {
    self.peek.replace(self.lexer.next_token())
  }

  fn peek(&self) -> Token {
    let peek = self.peek.take();
    self.peek.set(peek.clone());
    peek
  }
}
