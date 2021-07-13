mod helium;
mod rust;
use crate::env::Env;
use crate::error::Error;
use std::cell::RefCell;
use std::rc::Rc;

pub fn register(env: &Rc<RefCell<Env>>) -> Result<(), Vec<Error>> {
  rust::register(env);
  helium::register(env)
}
