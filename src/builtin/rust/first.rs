use super::helpers::{type_error, validate_params, Result};
use crate::object::{Object, Type};

pub fn first(args: Vec<Object>) -> Result {
  validate_params(&args, 1)?;

  Ok(match &args.get(0).unwrap().content {
    Type::String(string) => Object::new(Type::String(
      string.chars().nth(0).unwrap_or('\0').to_string(),
    )),
    Type::Array(array) => match array.get(0) {
      Some(obj) => obj.clone(),
      None => Object::NULL,
    },
    _ => return Err(type_error("array or string", args.get(0).unwrap())),
  })
}
