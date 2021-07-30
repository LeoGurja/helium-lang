mod helium;
mod rust;
use crate::{env::Env, error::Error};

pub fn register(env: &Env) -> Result<(), Vec<Error>> {
  rust::register(&env);
  helium::register(&env)
}
