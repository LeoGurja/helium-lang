use crate::{error::Error, helpers::validate_params, object::Object};

pub fn push(mut args: Vec<Object>) -> Result<Object, Error> {
  validate_params(&args, 2)?;
  match (args.remove(0), args.remove(0)) {
    (Object::Array(array), obj) => {
      let mut new_array = array.clone();
      new_array.push(obj.clone());
      Ok(Object::Array(new_array))
    }
    (Object::String(left), Object::String(right)) => {
      Ok(Object::String(format!("{}{}", left, right)))
    }
    (arr, _) => Err(Error::type_error("array or string", arr)),
  }
}
