use super::helpers::{type_error, validate_params, Result};
use crate::object::Object;

pub fn last(args: Vec<Object>) -> Result {
  validate_params(&args, 1)?;

  Ok(match &args.get(0).unwrap() {
    Object::String(string) => Object::String(string.chars().last().unwrap_or('\0').to_string()),
    Object::Array(array) => match array.last() {
      Some(obj) => obj.clone(),
      None => Object::Null,
    },
    _ => return Err(type_error("array or string", args.get(0).unwrap())),
  })
}
