use crate::{error::Error, object::Object};

pub fn validate_params(args: &[Object], size: usize) -> Result<(), Error> {
  if args.len() != size {
    Err(Error::wrong_parameters(size, args.len()))
  } else {
    Ok(())
  }
}
