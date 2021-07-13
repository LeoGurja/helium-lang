use crate::error::Error;
use crate::object::Object;

pub type Result = std::result::Result<Object, Error>;

pub fn type_error(expected: &str, got: &Object) -> Error {
  Error::TypeError(expected.to_owned(), got.clone())
}

pub fn validate_params(args: &Vec<Object>, size: usize) -> std::result::Result<(), Error> {
  if args.len() != size {
    return Err(Error::WrongParameters(size, args.len()));
  }

  Ok(())
}
