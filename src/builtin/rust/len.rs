use super::helpers::{type_error, validate_params, Result};
use crate::object::{Object, Type};

pub fn len(args: Vec<Object>) -> Result {
  validate_params(&args, 1)?;

  Ok(Object::new(match &args.get(0).unwrap().content {
    Type::String(string) => Type::Integer(string.len() as i64),
    Type::Array(array) => Type::Integer(array.len() as i64),
    _ => return Err(type_error("array or string", args.get(0).unwrap())),
  }))
}
