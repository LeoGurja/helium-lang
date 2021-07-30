use crate::{env::Env, error::Error, helium};

pub fn register(env: &Env) -> Result<(), Vec<Error>> {
  helium::import(env, "./src/builtin/helium/reduce.he")?;
  helium::import(env, "./src/builtin/helium/map.he")
}
