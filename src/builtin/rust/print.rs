use super::helpers::Result;
use crate::object::Object;

pub fn print(args: Vec<Object>) -> Result {
  for arg in args {
    println!("{}", arg);
  }
  Ok(Object::NULL)
}
