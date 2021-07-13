use crate::builtin;
use crate::errors::EvalError;
use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Env(Rc<RefCell<Environment>>);

#[derive(Debug, PartialEq)]
struct Environment {
  store: HashMap<String, Object>,
  parent: Option<Env>,
}

impl Env {
  fn new(enviroment: Environment) -> Self {
    Env(Rc::new(RefCell::new(enviroment)))
  }
  pub fn global() -> Self {
    let env = Env::new(Environment::new(None));
    builtin::register(&env).unwrap();
    env
  }

  pub fn local(parent: &Env) -> Self {
    Env::new(Environment::new(Some(parent.clone())))
  }

  pub fn get(&self, key: &str) -> Option<Object> {
    let env = self.0.borrow();
    match env.get(key) {
      Some(obj) => Some(obj),
      None => match &env.parent {
        Some(parent) => parent.get(key),
        None => None,
      },
    }
  }

  pub fn set(&self, key: &str, value: Object) {
    self.0.borrow_mut().set(key, value)
  }

  pub fn update(&self, key: &str, value: Object) -> Result<(), EvalError> {
    let mut env = self.0.borrow_mut();
    if env.contains_key(key) {
      env.set(key, value);
      Ok(())
    } else {
      match &mut env.parent {
        Some(parent) => parent.update(key, value),
        None => Err(EvalError::UndefinedVariable(key.to_owned())),
      }
    }
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

  pub fn contains_key(&self, key: &str) -> bool {
    self.store.contains_key(key)
  }
}
