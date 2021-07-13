mod first;
mod helpers;
mod last;
mod len;
mod push;
#[cfg(test)]
mod test;
use crate::env::Env;
use crate::object::{Object, Type};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register(env: &Rc<RefCell<Env>>) {
  add(env, "len", len::len);
  add(env, "first", first::first);
  add(env, "last", last::last);
  add(env, "push", push::push);
}

fn add(env: &Rc<RefCell<Env>>, name: &str, function: fn(Vec<Object>) -> helpers::Result) {
  env
    .borrow_mut()
    .set(name, Object::new(Type::BuiltIn(function)))
}
