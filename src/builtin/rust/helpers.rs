use crate::error::Error;
use crate::object::Object;

pub fn validate_params(args: &Vec<Object>, size: usize) -> Result<(), Error> {
  if args.len() != size {
    Err(Error::WrongParameters(size, args.len()))
  } else {
    Ok(())
  }
}
