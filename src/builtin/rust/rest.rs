use crate::{error::Error, helpers::validate_params, object::Object};

pub fn rest(mut args: Vec<Object>) -> Result<Object, Error> {
  validate_params(&args, 1)?;
  match args.remove(0) {
    Object::Array(values) => {
      if values.is_empty() {
        Ok(Object::Null)
      } else {
        Ok(Object::Array(values[1..].to_vec()))
      }
    }
    obj => Err(Error::type_error("array", obj)),
  }
}
