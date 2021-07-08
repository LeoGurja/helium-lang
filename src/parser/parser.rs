use super::parser_error::ParserError;
use crate::ast::{Block, Expression, Infix, Precedence, Prefix, Statement};
use crate::lexer::{Lexer, Token};

type Result<T> = std::result::Result<T, ParserError>;

pub struct Parser {
  lexer: Lexer,
  current: Token,
  pub errors: Vec<ParserError>,
}

impl Parser {
  pub fn new(input: String) -> Self {
    let mut lexer = Lexer::new(input);
    Parser {
      current: lexer.next_token(),
      lexer,
      errors: vec![],
    }
  }

  pub fn parse(&mut self) -> Block {
    let mut block = vec![];

    while self.current != Token::Eof {
      println!("statement: {:?}", self.current);
      match self.parse_statement() {
        Ok(statement) => {
          block.push(statement);
        }
        Err(err) => {
          self.errors.push(err);
        }
      }
    }
    block
  }

  fn parse_block(&mut self) -> Result<Block> {
    let mut block = vec![];

    self.eat(Token::LeftBrace)?;
    while self.current != Token::RightBrace {
      match self.parse_statement() {
        Ok(statement) => block.push(statement),
        Err(err) => self.errors.push(err),
      }
    }
    self.eat(Token::RightBrace)?;
    Ok(block)
  }

  fn parse_statement(&mut self) -> Result<Statement> {
    let statement = match self.current {
      Token::Let => self.parse_variable_declaration(),
      Token::Return => self.parse_return(),
      _ => Ok(Statement::Expression(
        self.parse_expression(Precedence::Lowest)?,
      )),
    };
    self.skip_semicolons();
    statement
  }

  fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression> {
    let mut left = match &self.current {
      Token::Id(value) => {
        let id = value.clone();
        self.parse_id(id)
      }
      Token::Integer(value) => {
        let integer = value.clone();
        self.parse_integer(integer)
      }
      Token::True | Token::False => self.parse_boolean(),
      Token::Bang | Token::Minus => self.parse_prefix_expression(),
      Token::LeftParen => self.parse_grouped_expression(),
      Token::If => self.parse_if_expression(),
      // Token::LeftBracket => Some(Parser::parse_array_literal),
      // Token::LeftBrace => Some(Parser::parse_hash_literal),
      Token::Function => self.parse_function(),
      _ => Err(ParserError::ExpectedExpression(self.current.clone())),
    };

    let mut operator = Infix::from(&self.current);
    while precedence < operator.0 {
      if let Some(infix) = operator.1 {
        left = self.parse_infix_expression(left?, infix, operator.0);
        operator = Infix::from(&self.current);
      } else {
        return left;
      }
    }
    left
  }

  fn parse_function(&mut self) -> Result<Expression> {
    self.advance();
    let name = match self.current.clone() {
      Token::Id(name) => {
        self.advance();
        Some(name.clone())
      }
      _ => None,
    };
    self.eat(Token::LeftParen)?;
    let mut args = vec![];
    while Token::RightParen != self.current {
      match &self.current {
        Token::Id(arg) => args.push(arg.clone()),
        _ => {
          return Err(ParserError::UnexpectedToken(
            Token::Id(String::from("...")),
            self.current.clone(),
          ));
        }
      }
      self.advance();
      if self.current == Token::Comma {
        self.advance();
      } else {
        break;
      }
    }
    self.eat(Token::RightParen)?;
    let block = self.parse_block()?;

    Ok(Expression::Function(name, args, block))
  }

  fn parse_if_expression(&mut self) -> Result<Expression> {
    self.advance();
    let condition = self.parse_expression(Precedence::Lowest)?;
    let consequence = self.parse_block()?;
    let alternative = if self.current == Token::Else {
      self.advance();
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

  fn parse_boolean(&mut self) -> Result<Expression> {
    let expression = Expression::Boolean(self.current == Token::True);
    self.advance();
    Ok(expression)
  }

  fn parse_grouped_expression(&mut self) -> Result<Expression> {
    self.advance();
    let left = self.parse_expression(Precedence::Lowest);
    self.eat(Token::RightParen)?;
    left
  }

  fn parse_id(&mut self, id: String) -> Result<Expression> {
    self.advance();
    if Token::LeftParen == self.current {
      return self.parse_function_call(id);
    }
    Ok(Expression::Id(id))
  }

  fn parse_function_call(&mut self, name: String) -> Result<Expression> {
    self.advance();
    let mut args = vec![];
    while self.current != Token::RightParen {
      args.push(self.parse_expression(Precedence::Lowest)?);
      if self.current == Token::Comma {
        self.advance();
      } else {
        break;
      }
    }
    self.eat(Token::RightParen)?;
    Ok(Expression::Call(name, args))
  }

  fn parse_integer(&mut self, integer: String) -> Result<Expression> {
    self.advance();
    match integer.parse() {
      Ok(value) => Ok(Expression::Integer(value)),
      Err(_) => Err(ParserError::ParsingError(
        String::from("integer"),
        integer.to_string(),
      )),
    }
  }

  fn parse_infix_expression(
    &mut self,
    left: Expression,
    infix: Infix,
    precedence: Precedence,
  ) -> Result<Expression> {
    self.advance();
    Ok(Expression::Infix(
      infix,
      Box::new(left),
      Box::new(self.parse_expression(precedence)?),
    ))
  }

  fn parse_prefix_expression(&mut self) -> Result<Expression> {
    let prefix = Prefix::from(&self.current)?;
    self.advance();
    let expression = self.parse_expression(Precedence::Prefix)?;

    Ok(Expression::Prefix(prefix, Box::new(expression)))
  }

  fn parse_variable_declaration(&mut self) -> Result<Statement> {
    let name;
    self.advance();
    if let Token::Id(id) = self.current.clone() {
      name = id;
    } else {
      return Err(ParserError::UnexpectedToken(
        Token::Id(String::from("...")),
        self.current.clone(),
      ));
    }

    self.advance();
    self.eat(Token::Assign)?;

    let value = self.parse_expression(Precedence::Lowest)?;

    Ok(Statement::Let(name, value))
  }

  fn parse_return(&mut self) -> Result<Statement> {
    self.advance();

    let value = self.parse_expression(Precedence::Lowest)?;

    Ok(Statement::Return(Some(value)))
  }

  fn skip_semicolons(&mut self) {
    while self.current == Token::Semicolon {
      self.advance();
    }
  }

  fn eat(&mut self, token: Token) -> Result<()> {
    if self.current != token {
      return Err(ParserError::UnexpectedToken(token, self.current.clone()));
    }
    self.advance();
    Ok(())
  }

  fn advance(&mut self) {
    self.current = self.lexer.next_token();
  }
}
