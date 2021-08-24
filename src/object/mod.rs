mod operations;

use crate::{ast::Statement, env::Env, error::Error, helpers::comma_separated};
use std::{collections::HashMap, fmt, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
  Array(Vec<Object>),
  Integer(i64),
  String(String),
  Return(Box<Object>),
  Function(Vec<String>, Rc<Statement>, Env),
  BuiltIn(fn(Vec<Object>) -> Result<Object, Error>),
  Hash(HashMap<String, Object>),
  True,
  False,
  Null,
}

impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Hash(hash) => format!("{:?}", hash),
        Self::Integer(value) => value.to_string(),
        Self::String(value) => format!("'{}'", value),
        Self::Return(obj) => obj.to_string(),
        Self::Function(args, ..) => format!("fn({})", comma_separated(args)),
        Self::BuiltIn(..) => format!("builtin fn()"),
        Self::Array(array) => format!("[{}]", comma_separated(array)),
        Self::True => "true".to_owned(),
        Self::False => "false".to_owned(),
        Self::Null => "null".to_owned(),
      }
    )
  }
}

impl Object {
  pub fn boolean(boolean: bool) -> Self {
    if boolean {
      Self::True
    } else {
      Self::False
    }
  }

  pub fn is_truthy(&self) -> bool {
    match self {
      Self::Null | Self::False => false,
      Self::Return(value) => value.is_truthy(),
      _ => true,
    }
  }
}
