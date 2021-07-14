use super::helpers::validate_params;
use crate::error::Error;
use crate::object::Object;

pub fn last(args: Vec<Object>) -> Result<Object, Error> {
  validate_params(&args, 1);

  match &args[0] {
    Object::String(string) => Ok(Object::String(
      string.chars().last().unwrap_or('\0').to_string(),
    )),
    Object::Array(array) => Ok(array.last().unwrap_or(&Object::Null).clone()),
    _ => Err(Error::TypeError(
      "array or string".to_owned(),
      args[0].clone(),
    )),
  }
}
