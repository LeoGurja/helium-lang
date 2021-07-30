mod first;
mod last;
mod len;
mod print;
mod push;
mod rest;
#[cfg(test)]
mod test;
use crate::{env::Env, error::Error, object::Object};

pub fn register(env: &Env) {
  let builtins: Vec<(&str, fn(Vec<Object>) -> Result<Object, Error>)> = vec![
    ("len", len::len),
    ("first", first::first),
    ("last", last::last),
    ("push", push::push),
    ("print", print::print),
    ("rest", rest::rest),
  ];

  for builtin in &builtins {
    env.set(builtin.0, Object::BuiltIn(builtin.1))
  }
}
