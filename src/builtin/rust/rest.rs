use super::helpers::{type_error, validate_params, Result};
use crate::object::Object;

pub fn rest(arguments: Vec<Object>) -> Result {
  validate_params(&arguments, 1)?;
  match &arguments[0] {
    Object::Array(values) => {
      if values.is_empty() {
        Ok(Object::Null)
      } else {
        Ok(Object::Array(values[1..].to_vec()))
      }
    }
    _ => Err(type_error("array", &arguments[0])),
  }
}
