use crate::object::{Object, Type};
use std::fmt;

pub trait UnwrappedPrintable: fmt::Debug + fmt::Display {}
impl<T: fmt::Debug + fmt::Display> UnwrappedPrintable for T {}

type Printable = Box<dyn UnwrappedPrintable>;

#[derive(Debug)]
pub enum EvalError {
  TypeMismatch(String, Type, Type),
  UnknownOperator(Printable, Object),
  UndefinedVariable(String),
  WrongParameters(usize, usize),
  CallError(Object),
  TypeError(String, Object),
  IndexError(Object, Object),
  CannotAssign(Object),
  UnsupportedHashKey(Type),
}

impl fmt::Display for EvalError {
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
    }
  }
}
