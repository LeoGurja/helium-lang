mod first;
mod helpers;
mod last;
mod len;
mod print;
mod push;
mod rest;
#[cfg(test)]
mod test;
use crate::env::Env;
use crate::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

pub fn register(env: &Rc<RefCell<Env>>) {
  add(env, "len", len::len);
  add(env, "first", first::first);
  add(env, "last", last::last);
  add(env, "push", push::push);
  add(env, "print", print::print);
  add(env, "rest", rest::rest);
}

fn add(env: &Rc<RefCell<Env>>, name: &str, function: fn(Vec<Object>) -> helpers::Result) {
  env.borrow_mut().set(name, Object::BuiltIn(function))
}
