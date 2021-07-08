use crate::builtin;
use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Link {
  fn global() -> Self;
  fn local(parent: &Env) -> Self;
  fn get(&self, key: &str) -> Option<Object>;
  fn set(&self, key: &str, value: Object);
}

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug, PartialEq)]
pub struct Environment {
  store: HashMap<String, Object>,
  parent: Option<Env>,
}

impl Link for Env {
  fn global() -> Self {
    let mut env = Rc::new(RefCell::new(Environment::new(None)));
    builtin::register(&mut env);
    env
  }

  fn local(parent: &Env) -> Self {
    Rc::new(RefCell::new(Environment::new(Some(parent.clone()))))
  }

  fn get(&self, key: &str) -> Option<Object> {
    self.borrow().get(key)
  }

  fn set(&self, key: &str, value: Object) {
    self.borrow_mut().set(key, value)
  }
}

impl Environment {
  pub fn new(parent: Option<Env>) -> Self {
    Self {
      store: HashMap::new(),
      parent,
    }
  }

  pub fn get(&self, key: &str) -> Option<Object> {
    match self.store.get(key) {
      Some(obj) => Some(obj.clone()),
      None => self.parent.as_ref().and_then(|parent| parent.get(key)),
    }
  }

  pub fn set(&mut self, key: &str, value: Object) {
    self.store.insert(key.to_string(), value);
  }
}
