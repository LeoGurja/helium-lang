use super::Object;
use crate::ast::Statement;
use crate::env::Env;
use crate::error::Error;
use crate::helpers::comma_separated;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
  Array(Vec<Object>),
  Integer(i64),
  String(String),
  Boolean(bool),
  Return(Box<Object>),
  Function(Vec<String>, Statement, Rc<RefCell<Env>>),
  BuiltIn(fn(Vec<Object>) -> Result<Object, Error>),
  Hash(HashMap<String, Object>),
  Null,
}

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Hash(hash) => format!("{:?}", hash),
        Self::Integer(value) => value.to_string(),
        Self::Boolean(value) => value.to_string(),
        Self::String(value) => value.clone(),
        Self::Return(obj) => obj.to_string(),
        Self::Function(args, ..) => format!("fn({})", comma_separated(args)),
        Self::BuiltIn(..) => format!("builtin fn()"),
        Self::Array(array) => format!("[{}]", comma_separated(array)),
        Self::Null => String::from("null"),
      }
    )
  }
}
