use super::helpers::validate_params;
use crate::error::Error;
use crate::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

pub fn rest(arguments: Vec<Object>) -> Result<Object, Error> {
  validate_params(&arguments, 1)?;
  match &arguments[0] {
    Object::Array(values) => {
      if values.borrow().is_empty() {
        Ok(Object::Null)
      } else {
        Ok(Object::Array(Rc::new(RefCell::new(
          values.borrow()[1..].to_vec(),
        ))))
      }
    }
    _ => Err(Error::TypeError("array".to_owned(), arguments[0].clone())),
  }
}
