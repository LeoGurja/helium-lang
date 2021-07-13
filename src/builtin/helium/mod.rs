use crate::env::Env;
use crate::error::Error;
use crate::helium;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub fn register(env: &Rc<RefCell<Env>>) -> Result<(), Vec<Error>> {
  let reduce = fs::read_to_string("./src/builtin/helium/reduce.he").unwrap();
  let map = fs::read_to_string("./src/builtin/helium/map.he").unwrap();
  helium::import(env, reduce)?;
  helium::import(env, map)
}
