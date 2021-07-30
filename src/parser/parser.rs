use crate::{
  ast::{Expression, Precedence, Statement},
  error::Error,
  lexer::{Lexer, Token},
};
use std::rc::Rc;

type Result<T> = std::result::Result<T, Error>;

pub struct Parser<'a> {
  lexer: Lexer<'a, Token<'a>>,
  current: Token<'a>,
  pub errors: Vec<Error>,
}

impl<'a> Parser<'a> {
  pub fn new(mut lexer: Lexer<'a, Token<'a>>) -> Self {
    Parser {
      current: lexer.next().unwrap_or(Token::Eof),
      lexer,
      errors: vec![],
    }
  }

  pub fn parse(&mut self) -> Vec<Statement> {
    let mut block = vec![];

    loop {
      match self.parse_statement() {
        Ok(Statement::Null) => break,
        Err(err) => self.errors.push(err),
        Ok(statement) => block.push(statement),
      }
    }

    block
  }

  fn parse_block(&mut self) -> Result<Statement> {
    let mut block = vec![];

    self.eat(Token::LeftBrace)?;
    loop {
      match self.parse_statement()? {
        Statement::Null => {
          self.eat(Token::RightBrace)?;
          return Ok(Statement::Block(block));
        }
        statement => block.push(statement),
      }
    }
  }

  fn parse_statement(&mut self) -> Result<Statement> {
    let statement = match self.current {
      Token::Let => self.parse_variable_declaration()?,
      Token::Return => self.parse_return()?,
      Token::For => self.parse_for_loop()?,
      Token::While => self.parse_while_loop()?,
      Token::LeftBrace => self.parse_block()?,
      Token::RightBrace | Token::Eof => Statement::Null,
      _ => Statement::Expression(self.parse_expression(Precedence::Lowest)?),
    };
    self.skip_semicolons();
    Ok(statement)
  }

  fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression> {
    let current = self.advance();
    let left = match current {
      Token::Id(value) => self.parse_id(value),
      Token::Integer(value) => Ok(Expression::Integer(value)),
      Token::String(value) => Ok(Expression::String(value.to_owned())),
      Token::True => Ok(Expression::TRUE),
      Token::False => Ok(Expression::FALSE),
      Token::Operator(op) => self.parse_prefix(op),
      Token::LeftParen => self.parse_grouped_expression(),
      Token::If => self.parse_if_expression(),
      Token::LeftBracket => Ok(Expression::Array(
        self.parse_expression_list(Token::RightBracket)?,
      )),
      Token::LeftBrace => self.parse_hash(),
      Token::Function => self.parse_function(),
      Token::Semicolon | Token::RightBrace => Ok(Expression::Null),
      token => Err(Error::expected_expression(token)),
    }?;

    self.parse_infix(left, precedence)
  }

  fn parse_hash(&mut self) -> Result<Expression> {
    let mut hash = vec![];
    while !self.eat_if(&Token::RightBrace) {
      let key = self.parse_expression(Precedence::Lowest)?;
      self.eat(Token::Colon)?;
      let value = self.parse_expression(Precedence::Lowest)?;
      hash.push((key, value));
      self.eat_if(&Token::Comma);
    }

    Ok(Expression::Hash(hash))
  }

  fn parse_infix(&mut self, mut left: Expression, precedence: Precedence) -> Result<Expression> {
    loop {
      left = match self.current {
        Token::LeftBracket => self.parse_index_expression(left)?,
        Token::LeftParen => self.parse_function_call(left)?,
        Token::Operator(operator) if precedence < Precedence::from(operator) => {
          self.parse_operator(left, operator)?
        }
        _ => break,
      };
    }
    Ok(left)
  }

  fn parse_operator(&mut self, left: Expression, operator: &'a str) -> Result<Expression> {
    self.advance();
    let precedence = Precedence::from(&operator);
    Ok(Expression::infix(
      operator,
      left,
      self.parse_expression(precedence)?,
    ))
  }

  fn parse_index_expression(&mut self, left: Expression) -> Result<Expression> {
    self.eat(Token::LeftBracket)?;
    let right = self.parse_expression(Precedence::Lowest)?;
    self.eat(Token::RightBracket)?;
    Ok(Expression::index(left, right))
  }

  fn parse_for_loop(&mut self) -> Result<Statement> {
    self.eat(Token::For)?;
    let variable = match self.advance() {
      Token::Id(id) => id,
      token => return Err(Error::unexpected_token(Token::Id("..."), token)),
    };
    self.eat(Token::In)?;

    Ok(Statement::for_loop(
      &variable,
      self.parse_expression(Precedence::Lowest)?,
      self.parse_statement()?,
    ))
  }

  fn parse_while_loop(&mut self) -> Result<Statement> {
    self.eat(Token::While)?;
    Ok(Statement::while_loop(
      self.parse_expression(Precedence::Lowest)?,
      self.parse_statement()?,
    ))
  }

  fn parse_function(&mut self) -> Result<Expression> {
    let mut args = vec![];

    let name = match self.advance() {
      Token::Id(id) => {
        self.eat(Token::LeftParen)?;
        Some(id.to_owned())
      }
      Token::LeftParen => None,
      token => return Err(Error::unexpected_token(Token::LeftParen, token)),
    };

    while !self.eat_if(&Token::RightParen) {
      match self.advance() {
        Token::Id(arg) => args.push(arg.to_owned()),
        token => return Err(Error::unexpected_token(Token::Id("..."), token)),
      };

      self.eat_if(&Token::Comma);
    }

    Ok(Expression::Function(
      name,
      args,
      Rc::new(self.parse_statement()?),
    ))
  }

  fn parse_expression_list(&mut self, end: Token) -> std::result::Result<Vec<Expression>, Error> {
    let mut args = vec![];
    while !self.eat_if(&end) {
      args.push(self.parse_expression(Precedence::Lowest)?);

      if !self.eat_if(&Token::Comma) {
        self.eat(end)?;
        break;
      }
    }
    Ok(args)
  }

  fn parse_if_expression(&mut self) -> Result<Expression> {
    let condition = self.parse_expression(Precedence::Lowest)?;
    let consequence = self.parse_statement()?;
    let alternative = if self.eat_if(&Token::Else) {
      let else_block = self.parse_statement()?;
      Some(else_block)
    } else {
      None
    };

    Ok(Expression::conditional(condition, consequence, alternative))
  }

  fn parse_grouped_expression(&mut self) -> Result<Expression> {
    let left = self.parse_expression(Precedence::Lowest);
    self.eat(Token::RightParen)?;
    left
  }

  fn parse_id(&mut self, id: &str) -> Result<Expression> {
    Ok(Expression::Id(id.to_owned()))
  }

  fn parse_function_call(&mut self, left: Expression) -> Result<Expression> {
    self.eat(Token::LeftParen)?;
    Ok(Expression::call(
      left,
      self.parse_expression_list(Token::RightParen)?,
    ))
  }

  fn parse_prefix(&mut self, operator: &str) -> Result<Expression> {
    Ok(Expression::prefix(
      operator,
      self.parse_expression(Precedence::Prefix)?,
    ))
  }

  fn parse_variable_declaration(&mut self) -> Result<Statement> {
    self.eat(Token::Let)?;
    let name = match self.advance() {
      Token::Id(id) => id.to_owned(),
      token => return Err(Error::unexpected_token(Token::Id("..."), token)),
    };

    self.eat(Token::Operator("="))?;

    let value = self.parse_expression(Precedence::Lowest)?;

    Ok(Statement::VariableDeclaration(name.to_owned(), value))
  }

  fn parse_return(&mut self) -> Result<Statement> {
    self.eat(Token::Return)?;
    let value = self.parse_expression(Precedence::Lowest)?;

    Ok(Statement::Return(value))
  }

  fn skip_semicolons(&mut self) {
    while self.eat_if(&Token::Semicolon) {}
  }

  fn eat(&mut self, should_be: Token) -> Result<()> {
    let current = self.advance();
    if current == should_be {
      Ok(())
    } else {
      Err(Error::unexpected_token(should_be, current))
    }
  }

  fn eat_if(&mut self, should_be: &Token) -> bool {
    if &self.current == should_be {
      self.advance();
      true
    } else {
      false
    }
  }

  fn advance<'s>(&'s mut self) -> Token<'a> {
    std::mem::replace(&mut self.current, self.lexer.next().unwrap_or(Token::Eof))
  }
}
