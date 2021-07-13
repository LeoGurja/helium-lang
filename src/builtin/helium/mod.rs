use crate::env::Env;
use crate::errors::Error;
use crate::helium;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub fn register(env: &Rc<RefCell<Env>>) -> Result<(), Error> {
  let source = fs::read_to_string("./src/builtin/helium/reduce.he").unwrap();
  helium::import(env, source)
}
