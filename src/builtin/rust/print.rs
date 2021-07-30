use crate::{error::Error, object::Object};

pub fn print(args: Vec<Object>) -> Result<Object, Error> {
  for arg in args {
    println!("{}", arg);
  }
  Ok(Object::Null)
}
