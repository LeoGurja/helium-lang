use crate::{error::Error, helpers::validate_params, object::Object};

pub fn first(mut args: Vec<Object>) -> Result<Object, Error> {
  validate_params(&args, 1)?;

  match args.remove(0) {
    Object::String(obj) if obj.len() == 0 => Ok(Object::Null),
    Object::Array(obj) if obj.len() == 0 => Ok(Object::Null),
    Object::String(string) => Ok(Object::String(string[0..1].to_string())),
    Object::Array(array) => match array.get(0) {
      Some(obj) => Ok(obj.clone()),
      None => Ok(Object::Null),
    },
    obj => Err(Error::type_error("array or string", obj)),
  }
}
