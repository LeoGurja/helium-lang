use crate::ast::Expression;
use crate::object::{Object, Type};
use crate::token::{Operator, Token};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
  UnexpectedToken(Token, Token),
  ExpectedExpression(Token),
  ExpectedPrefix(Token),
  ParsingError(String, String),
  ExpectedId(Expression),
  TypeMismatch(String, Type, Type),
  UnknownOperator(Operator, Object),
  UndefinedVariable(String),
  WrongParameters(usize, usize),
  CallError(Object),
  TypeError(String, Object),
  IndexError(Object, Object),
  CannotAssign(Object),
  UnsupportedHashKey(Type),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::UnsupportedHashKey(obj) => {
        write!(f, "UnsupportHashKey:\n\t{} is not a valid key", obj)
      }
      Self::CannotAssign(obj) => {
        write!(f, "CannotAssign:\n\tcannot assign to {}", obj)
      }
      Self::IndexError(obj, indexer) => {
        write!(
          f,
          "IndexError:\n\t{} cannot be acessed by indexed by {}",
          obj, indexer
        )
      }
      Self::TypeError(expected, got) => {
        write!(
          f,
          "TypeError:\n\texpected a {}, got {:?} instead",
          expected, got
        )
      }
      Self::TypeMismatch(operator, left, right) => {
        write!(f, "TypeMismatch:\n\t{:?} {} {:?}", left, operator, right)
      }
      Self::UnknownOperator(operator, obj_type) => {
        write!(
          f,
          "UnknownOperator:\n\tcan't use {} on {}",
          operator, obj_type
        )
      }
      Self::UndefinedVariable(name) => {
        write!(
          f,
          "UndefinedVariable:\n\t{} was used before it was defined",
          name
        )
      }
      Self::WrongParameters(expected, got) => {
        write!(
          f,
          "WrongParameters:\n\texpected {} parameters, got {} instead",
          expected, got
        )
      }
      Self::CallError(name) => {
        write!(f, "CallError:\n\t{} is not a function", name)
      }
      Self::ExpectedId(got) => {
        write!(f, "ExpectedId:\n\tExpected an id, got {} instead", got)
      }
      Self::UnexpectedToken(expected, got) => {
        write!(
          f,
          "UnexpectedToken:\n\tExpected {}, got {:?} instead",
          expected, got
        )
      }
      Self::ParsingError(expected, got) => {
        write!(
          f,
          "ParsingError:\n\tExpected a valid {}, got {} instead",
          expected, got
        )
      }
      Self::ExpectedExpression(got) => {
        write!(
          f,
          "ExpectedExpression:\n\tExpected an expression, got {:?} instead",
          got
        )
      }
      Self::ExpectedPrefix(got) => {
        write!(
          f,
          "ExpectedPrefix:\n\tExpected a prefix, got {:?} instead",
          got
        )
      }
    }
  }
}
