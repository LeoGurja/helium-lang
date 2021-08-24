use crate::{builtin, object::Object};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type Env = Rc<Environment>;

pub fn global() -> Env {
  let env = Rc::new(Environment {
    store: RefCell::new(HashMap::new()),
    parent: None,
  });

  builtin::register(&env).unwrap();
  env
}

pub fn local(parent: Env) -> Env {
  Rc::new(Environment {
    store: RefCell::new(HashMap::new()),
    parent: Some(parent),
  })
}

#[derive(Debug, PartialEq)]
pub struct Environment {
  store: RefCell<HashMap<String, Object>>,
  parent: Option<Rc<Self>>,
}

impl Environment {
  pub fn get(&self, key: &str) -> Option<Object> {
    match self.store.borrow().get(key) {
      None => match &self.parent {
        Some(parent) => parent.get(key),
        None => None,
      },
      some => some.cloned(),
    }
  }

  pub fn set(&self, key: &str, value: Object) {
    self.store.borrow_mut().insert(key.to_owned(), value);
  }
}
