mod first;
mod helpers;
mod last;
mod len;
mod push;
#[cfg(test)]
mod test;
use crate::object::Object;
use crate::visitor::{Env, Link, Result};

pub fn register(env: &mut Env) {
  add(env, "len", len::len);
  add(env, "first", first::first);
  add(env, "last", last::last);
  add(env, "push", push::push);
}

fn add(env: &mut Env, name: &str, function: fn(Vec<Object>) -> Result) {
  env.set(name, Object::BuiltIn(function))
}
