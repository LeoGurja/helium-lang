use super::helpers::validate_params;
use crate::error::Error;
use crate::object::Object;

pub fn rest(arguments: Vec<Object>) -> Result<Object, Error> {
  validate_params(&arguments, 1);
  match &arguments[0] {
    Object::Array(values) => {
      if values.is_empty() {
        Ok(Object::Null)
      } else {
        Ok(Object::Array(values[1..].to_vec()))
      }
    }
    _ => Err(Error::TypeError("array".to_owned(), arguments[0].clone())),
  }
}
