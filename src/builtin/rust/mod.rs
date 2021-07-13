mod first;
mod helpers;
mod last;
mod len;
mod push;
#[cfg(test)]
mod test;
use crate::env::Env;
use crate::object::{Object, Type};

pub fn register(env: &Env) {
  add(env, "len", len::len);
  add(env, "first", first::first);
  add(env, "last", last::last);
  add(env, "push", push::push);
}

fn add(env: &Env, name: &str, function: fn(Vec<Object>) -> helpers::Result) {
  env.set(name, Object::new(Type::BuiltIn(function)))
}
