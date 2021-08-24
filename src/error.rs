use crate::{ast::Expression, lexer::Token, object::Object};
use std::{fmt, process::exit};

#[derive(Debug, PartialEq)]
pub struct Error {
  message: String,
  _type: ErrorType,
}

#[derive(Debug, PartialEq)]
pub enum ErrorType {
  UnexpectedToken,
  ExpectedExpression,
  ExpectedPrefix,
  ExpectedId,
  TypeMismatch,
  UnknownOperator,
  UndefinedVariable,
  WrongParameters,
  CallError,
  TypeError,
  IndexError,
  CannotAssign,
}

impl Error {
  pub fn raise(&self) -> ! {
    eprintln!("{}", self);
    exit(1)
  }

  pub fn unexpected_token(expected: Token, got: Token) -> Self {
    Self {
      message: format!("expected {:?}, got {:?} instead", expected, got),
      _type: ErrorType::UnexpectedToken,
    }
  }

  pub fn expected_expression(got: Token) -> Self {
    Self {
      message: format!("expected an expression, got {:?} instead", got),
      _type: ErrorType::ExpectedExpression,
    }
  }

  pub fn expected_prefix(got: Token) -> Self {
    Self {
      message: format!("expected a prefix, got {:?} instead", got),
      _type: ErrorType::ExpectedPrefix,
    }
  }

  pub fn type_mismatch(operator: &str, left: Object, right: Object) -> Self {
    Self {
      message: format!("cannot use '{}' on {} and {}", operator, left, right),
      _type: ErrorType::TypeMismatch,
    }
  }

  pub fn unknown_operator(operator: String, obj: Object) -> Self {
    Self {
      message: format!("cannot use '{}' on {}", operator, obj),
      _type: ErrorType::UnknownOperator,
    }
  }

  pub fn undefined_variable(id: &str) -> Self {
    Self {
      message: format!("'{}' was used before it was defined", id),
      _type: ErrorType::UndefinedVariable,
    }
  }

  pub fn wrong_parameters(expected: usize, got: usize) -> Self {
    Self {
      message: format!("expected {} parameters, got {} instead", expected, got),
      _type: ErrorType::WrongParameters,
    }
  }

  pub fn call_error(obj: Object) -> Self {
    Self {
      message: format!("{} is not a function", obj),
      _type: ErrorType::CallError,
    }
  }

  pub fn type_error(expected: &str, got: Object) -> Self {
    Self {
      message: format!("expected {:?}, got {:?} instead", expected, got),
      _type: ErrorType::TypeError,
    }
  }

  pub fn index_error(iterable: Object, index: Object) -> Self {
    Self {
      message: format!("cannot index {} with {:?}", iterable, index),
      _type: ErrorType::IndexError,
    }
  }

  pub fn cannot_assign(exp: Expression) -> Self {
    Self {
      message: format!("cannot assign to {:?}", exp),
      _type: ErrorType::UnexpectedToken,
    }
  }

  fn type_string(&self) -> &str {
    match self._type {
      ErrorType::CallError => "CallError",
      ErrorType::IndexError => "IndexError",
      ErrorType::TypeError => "TypeError",
      ErrorType::CannotAssign => "CannotAssign",
      ErrorType::ExpectedExpression => "ExpectedExpression",
      ErrorType::ExpectedId => "ExpectedId",
      ErrorType::ExpectedPrefix => "ExpectedPrefix",
      ErrorType::TypeMismatch => "TypeMismatch",
      ErrorType::UndefinedVariable => "UndefinedVariable",
      ErrorType::UnexpectedToken => "UnexpectedToken",
      ErrorType::UnknownOperator => "UnknownOperator",
      ErrorType::WrongParameters => "WrongParameters",
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}:\n\t{}", self.type_string(), self.message)
  }
}
