use super::helpers::validate_params;
use crate::error::Error;
use crate::object::Object;

pub fn len(args: Vec<Object>) -> Result<Object, Error> {
  validate_params(&args, 1)?;

  match &args[0] {
    Object::String(string) => Ok(Object::Integer(string.len() as i64)),
    Object::Array(array) => Ok(Object::Integer(array.borrow().len() as i64)),
    _ => Err(Error::TypeError(
      "array or string".to_owned(),
      args[0].clone(),
    )),
  }
}
