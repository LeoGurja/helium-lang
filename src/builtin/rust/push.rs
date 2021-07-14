use super::helpers::validate_params;
use crate::error::Error;
use crate::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

pub fn push(args: Vec<Object>) -> Result<Object, Error> {
  validate_params(&args, 2)?;

  match &args[0] {
    Object::Array(array) => {
      let mut new_array = array.borrow().clone();
      new_array.push(args[1].clone());
      Ok(Object::Array(Rc::new(RefCell::new(new_array))))
    }
    Object::String(string) => {
      let mut new_string = string.clone();
      new_string.push_str(&args[1].to_string());
      Ok(Object::String(new_string))
    }
    _ => Err(Error::TypeError(
      "array or string".to_owned(),
      args[0].clone(),
    )),
  }
}
