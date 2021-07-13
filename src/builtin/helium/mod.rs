use crate::env::Env;
use crate::errors::Error;
use crate::helium;
use std::fs;

pub fn register(env: &Env) -> Result<(), Error> {
  let source = fs::read_to_string("./src/builtin/helium/reduce.he").unwrap();
  helium::import(env, source)
}
