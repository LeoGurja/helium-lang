use crate::{error::Error, helpers::validate_params, object::Object};

pub fn last(mut args: Vec<Object>) -> Result<Object, Error> {
  validate_params(&args, 1)?;

  match args.remove(0) {
    Object::String(obj) if obj.len() == 0 => Ok(Object::Null),
    Object::Array(obj) if obj.len() == 0 => Ok(Object::Null),
    Object::String(string) => Ok(Object::String(string[string.len() - 1..].to_string())),
    Object::Array(array) => Ok(array.last().unwrap_or(&Object::Null).clone()),
    obj => Err(Error::type_error("array or string", obj)),
  }
}
