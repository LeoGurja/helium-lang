mod helpers;
mod len;
use crate::object::Object;
use crate::visitor::{Env, Link, Result};

pub fn register(env: &mut Env) {
  add(env, "len", len::len);
}

fn add(env: &mut Env, name: &str, function: fn(Vec<Object>) -> Result) {
  env.set(name, Object::BuiltIn(function))
}
