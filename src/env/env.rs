use crate::builtin;
use crate::errors::EvalError;
use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Env {
  store: HashMap<String, Object>,
  parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
  fn new(parent: Option<Rc<RefCell<Env>>>) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Env {
      store: HashMap::new(),
      parent,
    }))
  }
  pub fn global() -> Rc<RefCell<Self>> {
    let env = Env::new(None);
    builtin::register(&env).unwrap();
    env
  }

  pub fn local(parent: Rc<RefCell<Env>>) -> Rc<RefCell<Self>> {
    Env::new(Some(parent))
  }

  pub fn get(&self, key: &str) -> Option<Object> {
    match self.store.get(key) {
      Some(obj) => Some(obj.clone()),
      None => match &self.parent {
        Some(parent) => parent.borrow().get(key),
        None => None,
      },
    }
  }

  pub fn update(&mut self, key: &str, value: Object) -> Result<(), EvalError> {
    if self.store.contains_key(key) {
      Ok(self.set(key, value))
    } else {
      match &self.parent {
        Some(parent) => parent.borrow_mut().update(key, value),
        None => Err(EvalError::UndefinedVariable(key.to_owned())),
      }
    }
  }

  pub fn set(&mut self, key: &str, value: Object) {
    self.store.insert(key.to_lowercase(), value);
  }
}
