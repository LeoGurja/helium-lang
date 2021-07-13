use crate::ast::{Expression, Precedence, Statement};
use crate::errors::ParserError;
use crate::lexer::Lexer;
use crate::token::{Operator, Token};
use std::cell::Cell;

type Result<T> = std::result::Result<T, ParserError>;

pub struct Parser {
  lexer: Lexer,
  current: Cell<Token>,
  errors: Vec<ParserError>,
}

impl Parser {
  pub fn new(lexer: Lexer) -> Self {
    Parser {
      current: Cell::new(lexer.next_token()),
      lexer,
      errors: vec![],
    }
  }

  pub fn parse(&mut self) -> std::result::Result<Vec<Statement>, Vec<ParserError>> {
    let mut block = vec![];

    loop {
      match self.parse_statement() {
        Ok(Statement::Null) => break,
        Err(err) => self.errors.push(err),
        Ok(statement) => block.push(statement),
      }
    }

    if self.errors.len() != 0 {
      Err(self.errors.clone())
    } else {
      Ok(block)
    }
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
    let statement = match self.current.get_mut() {
      Token::Let => self.parse_variable_declaration(),
      Token::Return => self.parse_return(),
      Token::For => self.parse_for_loop(),
      Token::While => self.parse_while_loop(),
      Token::LeftBrace => return self.parse_block(),
      Token::Eof | Token::RightBrace => Ok(Statement::Null),
      _ => Ok(Statement::Expression(
        self.parse_expression(Precedence::Lowest)?,
      )),
    };
    self.skip_semicolons();
    statement
  }

  fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression> {
    let left = match self.advance() {
      Token::Id(value) => self.parse_id(value),
      Token::Integer(value) => self.parse_integer(value),
      Token::String(value) => Ok(Expression::String(value.clone())),
      Token::True => Ok(Expression::TRUE),
      Token::False => Ok(Expression::FALSE),
      Token::Operator(operator) => self.parse_prefix(operator),
      Token::LeftParen => self.parse_grouped_expression(),
      Token::If => self.parse_if_expression(),
      Token::LeftBracket => Ok(Expression::Array(
        self.parse_expression_list(Token::RightBracket)?,
      )),
      Token::LeftBrace => self.parse_hash(),
      Token::Function => self.parse_function(),
      token => Err(ParserError::ExpectedExpression(token)),
    }?;

    self.parse_infix(left, precedence)
  }

  fn parse_hash(&mut self) -> Result<Expression> {
    let mut hash = vec![];
    while !self.eat_if(Token::RightBrace) {
      let key = self.parse_expression(Precedence::Lowest)?;
      self.eat(Token::Colon)?;
      let value = self.parse_expression(Precedence::Lowest)?;
      hash.push((key, value));
      self.eat_if(Token::Comma);
    }

    Ok(Expression::Hash(hash))
  }

  fn parse_infix(&mut self, mut left: Expression, precedence: Precedence) -> Result<Expression> {
    loop {
      left = match &self.current.get_mut().clone() {
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

  fn parse_operator(&mut self, left: Expression, operator: &Operator) -> Result<Expression> {
    self.advance();
    Ok(Expression::Infix(
      operator.clone(),
      Box::new(left),
      Box::new(self.parse_expression(Precedence::from(operator))?),
    ))
  }

  fn parse_index_expression(&mut self, left: Expression) -> Result<Expression> {
    self.eat(Token::LeftBracket)?;
    let right = self.parse_expression(Precedence::Lowest)?;
    self.eat(Token::RightBracket)?;
    Ok(Expression::Index(Box::new(left), Box::new(right)))
  }

  fn parse_for_loop(&mut self) -> Result<Statement> {
    self.eat(Token::For)?;
    let variable = match self.advance() {
      Token::Id(id) => id,
      token => {
        return Err(ParserError::UnexpectedToken(
          Token::Id(String::from("..")),
          token,
        ))
      }
    };
    self.eat(Token::In)?;

    Ok(Statement::For(
      variable.clone(),
      self.parse_expression(Precedence::Lowest)?,
      Box::new(self.parse_statement()?),
    ))
  }

  fn parse_while_loop(&mut self) -> Result<Statement> {
    self.eat(Token::While)?;
    Ok(Statement::While(
      self.parse_expression(Precedence::Lowest)?,
      Box::new(self.parse_statement()?),
    ))
  }

  fn parse_function(&mut self) -> Result<Expression> {
    let mut args = vec![];

    let name = match self.advance() {
      Token::Id(id) => {
        self.eat(Token::LeftParen)?;
        Some(id)
      }
      Token::LeftParen => None,
      token => return Err(ParserError::UnexpectedToken(Token::LeftParen, token)),
    };

    while !self.eat_if(Token::RightParen) {
      match self.advance() {
        Token::Id(arg) => args.push(arg),
        token => {
          return Err(ParserError::UnexpectedToken(
            Token::Id(String::from("...")),
            token,
          ))
        }
      };

      self.eat_if(Token::Comma);
    }

    Ok(Expression::Function(
      name,
      args,
      Box::new(self.parse_statement()?),
    ))
  }

  fn parse_expression_list(
    &mut self,
    end: Token,
  ) -> std::result::Result<Vec<Expression>, ParserError> {
    let mut args = vec![];
    loop {
      args.push(self.parse_expression(Precedence::Lowest)?);

      if !self.eat_if(Token::Comma) {
        self.eat(end)?;
        return Ok(args);
      }
    }
  }

  fn parse_if_expression(&mut self) -> Result<Expression> {
    let condition = Box::new(self.parse_expression(Precedence::Lowest)?);
    let consequence = Box::new(self.parse_statement()?);
    let alternative = if self.eat_if(Token::Else) {
      let else_block = Box::new(self.parse_statement()?);
      Some(else_block)
    } else {
      None
    };

    Ok(Expression::If(condition, consequence, alternative))
  }

  fn parse_grouped_expression(&mut self) -> Result<Expression> {
    let left = self.parse_expression(Precedence::Lowest);
    self.eat(Token::RightParen)?;
    left
  }

  fn parse_id(&mut self, id: String) -> Result<Expression> {
    Ok(Expression::Id(id))
  }

  fn parse_function_call(&mut self, left: Expression) -> Result<Expression> {
    self.eat(Token::LeftParen)?;
    Ok(Expression::Call(
      Box::new(left),
      self.parse_expression_list(Token::RightParen)?,
    ))
  }

  fn parse_integer(&mut self, integer: String) -> Result<Expression> {
    match integer.parse() {
      Ok(value) => Ok(Expression::Integer(value)),
      Err(_) => Err(ParserError::ParsingError(String::from("integer"), integer)),
    }
  }

  fn parse_prefix(&mut self, operator: Operator) -> Result<Expression> {
    let expression = self.parse_expression(Precedence::Prefix)?;

    Ok(Expression::Prefix(operator, Box::new(expression)))
  }

  fn parse_variable_declaration(&mut self) -> Result<Statement> {
    self.eat(Token::Let)?;
    let name = match self.advance() {
      Token::Id(id) => id.clone(),
      token => {
        return Err(ParserError::UnexpectedToken(
          Token::Id(String::from("...")),
          token,
        ))
      }
    };

    self.eat(Token::Operator(Operator::Assign))?;

    let value = self.parse_expression(Precedence::Lowest)?;

    Ok(Statement::Let(name, value))
  }

  fn parse_return(&mut self) -> Result<Statement> {
    self.eat(Token::Return)?;
    let value = self.parse_expression(Precedence::Lowest)?;

    Ok(Statement::Return(Some(value)))
  }

  fn skip_semicolons(&mut self) {
    while self.eat_if(Token::Semicolon) {}
  }

  fn eat(&self, should_be: Token) -> Result<Token> {
    let token = self.advance();
    if token == should_be {
      Ok(should_be)
    } else {
      Err(ParserError::UnexpectedToken(should_be, token))
    }
  }

  fn eat_if(&mut self, should_be: Token) -> bool {
    if self.current.get_mut() == &should_be {
      self.advance();
      true
    } else {
      false
    }
  }

  fn advance(&self) -> Token {
    self.current.replace(self.lexer.next_token())
  }
}
