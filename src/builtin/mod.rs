mod helium;
mod rust;
use crate::env::Env;
use crate::errors::Error;

pub fn register(env: &Env) -> Result<(), Error> {
  rust::register(env);
  helium::register(env)
}
