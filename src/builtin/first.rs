use super::helpers::{type_error, validate_params};
use crate::object::Object;
use crate::visitor::Result;

pub fn first(args: Vec<Object>) -> Result {
  validate_params(&args, 1)?;

  Ok(match args.get(0).unwrap() {
    Object::String(string) => Object::String(string.chars().nth(0).unwrap_or('\0').to_string()),
    Object::Array(array) => array.get(0).unwrap_or(&Object::Null).clone(),
    _ => return Err(type_error("string", args.get(0).unwrap())),
  })
}
