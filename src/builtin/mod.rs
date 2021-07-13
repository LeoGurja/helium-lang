mod helium;
mod rust;
use crate::env::Env;
use crate::errors::Error;
use std::cell::RefCell;
use std::rc::Rc;

pub fn register(env: &Rc<RefCell<Env>>) -> Result<(), Error> {
  rust::register(env);
  helium::register(env)
}
