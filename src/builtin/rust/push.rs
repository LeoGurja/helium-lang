use super::helpers::{type_error, validate_params, Result};
use crate::object::Object;

pub fn push(args: Vec<Object>) -> Result {
  validate_params(&args, 2)?;

  let obj = args.get(0).unwrap();
  let pushed = args.get(1).unwrap().clone();
  Ok(match &obj {
    Object::Array(array) => {
      let mut new_array = array.clone();
      new_array.push(pushed);
      Object::Array(new_array)
    }
    Object::String(string) => {
      let mut new_string = string.clone();
      new_string.push_str(&pushed.to_string());
      Object::String(new_string)
    }
    _ => return Err(type_error("array or string", args.get(0).unwrap())),
  })
}
