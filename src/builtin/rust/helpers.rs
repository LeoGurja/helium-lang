use crate::errors::EvalError;
use crate::object::Object;

pub type Result = std::result::Result<Object, EvalError>;

pub fn type_error(expected: &str, got: &Object) -> EvalError {
  EvalError::TypeError(expected.to_owned(), got.clone())
}

pub fn validate_params(args: &Vec<Object>, size: usize) -> std::result::Result<(), EvalError> {
  if args.len() != size {
    return Err(EvalError::WrongParameters(size, args.len()));
  }

  Ok(())
}
