use super::helpers::{type_error, validate_params};
use crate::object::Object;
use crate::visitor::Result;

pub fn last(args: Vec<Object>) -> Result {
  validate_params(&args, 1)?;

  Ok(match args.get(0).unwrap() {
    Object::String(string) => Object::String(string.chars().last().unwrap_or('\0').to_string()),
    Object::Array(array) => array.last().unwrap_or(&Object::Null).clone(),
    _ => return Err(type_error("array or string", args.get(0).unwrap())),
  })
}
