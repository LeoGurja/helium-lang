use crate::ast::Block;
use crate::helpers::comma_separated;
use crate::visitor::{Env, EvalError, Result};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
  Array(Vec<Object>),
  Integer(i64),
  String(String),
  Boolean(bool),
  Return(Box<Object>),
  Function(Vec<String>, Block, Env),
  BuiltIn(fn(Vec<Object>) -> Result),
  Null,
}

impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl Object {
  pub const TRUE: Object = Object::Boolean(true);
  pub const FALSE: Object = Object::Boolean(false);

  fn as_string(&self) -> String {
    match self {
      Self::Integer(value) => value.to_string(),
      Self::Boolean(value) => value.to_string(),
      Self::String(value) => value.clone(),
      Self::Return(obj) => obj.as_string(),
      Self::Function(args, ..) => format!("fn({})", comma_separated(args)),
      Self::BuiltIn(..) => format!("builtin fn()"),
      Self::Array(array) => format!("[{}]", comma_separated(array)),
      Self::Null => String::from("null"),
    }
  }

  pub fn not(self) -> Object {
    match self {
      Object::Boolean(true) => Object::FALSE,
      Object::Boolean(false) | Object::Null => Object::TRUE,
      _ => Object::FALSE,
    }
  }

  pub fn negative(self) -> Result {
    match self {
      Object::Integer(value) => Ok(Object::Integer(-value)),
      _ => Err(EvalError::UnknownOperator(
        Box::new(String::from("-")),
        self,
      )),
    }
  }

  pub fn add(self, obj: Object) -> Result {
    Ok(match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Object::Integer(left + right),
      (Object::String(left), Object::String(right)) => Object::String(left + &right),
      (left, right) => return Err(EvalError::TypeMismatch(String::from("+"), left, right)),
    })
  }

  pub fn subtract(self, obj: Object) -> Result {
    Ok(match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Object::Integer(left - right),
      (left, right) => return Err(EvalError::TypeMismatch(String::from("-"), left, right)),
    })
  }

  pub fn multiply(self, obj: Object) -> Result {
    Ok(match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Object::Integer(left * right),
      (left, right) => return Err(EvalError::TypeMismatch(String::from("*"), left, right)),
    })
  }

  pub fn divide(self, obj: Object) -> Result {
    Ok(match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Object::Integer(left / right),
      (left, right) => return Err(EvalError::TypeMismatch(String::from("/"), left, right)),
    })
  }

  pub fn greater_than(self, obj: Object) -> Result {
    Ok(match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => {
        if left > right {
          Object::TRUE
        } else {
          Object::FALSE
        }
      }
      (left, right) => return Err(EvalError::TypeMismatch(String::from(">"), left, right)),
    })
  }

  pub fn less_than(self, obj: Object) -> Result {
    Ok(match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => {
        if left < right {
          Object::TRUE
        } else {
          Object::FALSE
        }
      }
      (left, right) => return Err(EvalError::TypeMismatch(String::from("<"), left, right)),
    })
  }
}
