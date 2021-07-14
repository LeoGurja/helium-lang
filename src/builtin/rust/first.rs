use super::helpers::validate_params;
use crate::error::Error;
use crate::object::Object;

pub fn first(args: Vec<Object>) -> Result<Object, Error> {
  validate_params(&args, 1);

  match &args[0] {
    Object::String(string) => Ok(Object::String(
      string.chars().nth(0).unwrap_or('\0').to_string(),
    )),
    Object::Array(array) => match array.get(0) {
      Some(obj) => Ok(obj.clone()),
      None => Ok(Object::Null),
    },
    _ => Err(Error::TypeError(
      "array or string".to_owned(),
      args[0].clone(),
    )),
  }
}
