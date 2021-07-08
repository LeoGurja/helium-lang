use super::helpers::{type_error, validate_params};
use crate::object::Object;
use crate::visitor::Result;

pub fn len(args: Vec<Object>) -> Result {
  validate_params(&args, 1)?;

  Ok(match args.get(0).unwrap() {
    Object::String(string) => Object::Integer(string.len() as i64),
    _ => return Err(type_error("string", args.get(0).unwrap())),
  })
}
