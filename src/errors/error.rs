use super::{EvalError, ParserError};

#[derive(Debug)]
pub enum Error {
  ParserError(Vec<ParserError>),
  EvalError(EvalError),
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::EvalError(err) => write!(f, "{}", err),
      Error::ParserError(errs) => write!(f, "{:?}", errs),
    }
  }
}
