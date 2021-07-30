use crate::{error::Error, helpers::validate_params, object::Object};

pub fn len(mut args: Vec<Object>) -> Result<Object, Error> {
  validate_params(&args, 1)?;

  match args.remove(0) {
    Object::String(string) => Ok(Object::Integer(string.len() as i64)),
    Object::Array(array) => Ok(Object::Integer(array.len() as i64)),
    obj => Err(Error::type_error("array or string", obj)),
  }
}
