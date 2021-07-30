use crate::{ast::Statement, env::Env, error::Error, helpers::comma_separated};
use std::{collections::HashMap, fmt, ops, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
  Array(Vec<Object>),
  Integer(i64),
  String(String),
  Boolean(bool),
  Return(Box<Object>),
  Function(Vec<String>, Rc<Statement>, Env),
  BuiltIn(fn(Vec<Object>) -> Result<Object, Error>),
  Hash(HashMap<String, Object>),
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
        Self::Boolean(value) => value.to_string(),
        Self::String(value) => format!("'{}'", value),
        Self::Return(obj) => obj.to_string(),
        Self::Function(args, ..) => format!("fn({})", comma_separated(args)),
        Self::BuiltIn(..) => format!("builtin fn()"),
        Self::Array(array) => format!("[{}]", comma_separated(array)),
        Self::Null => "null".to_string(),
      }
    )
  }
}

impl ops::Add for Object {
  type Output = Object;

  fn add(self, obj: Object) -> Self::Output {
    match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Object::Integer(left + right),
      (Object::String(left), Object::String(right)) => Object::String(format!("{}{}", left, right)),
      (left, right) => Error::type_mismatch("+", left, right).raise(),
    }
  }
}

impl ops::Sub for Object {
  type Output = Object;

  fn sub(self, obj: Object) -> Self::Output {
    match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Object::Integer(left - right),
      (left, right) => Error::type_mismatch("-", left, right).raise(),
    }
  }
}

impl ops::Div for Object {
  type Output = Object;

  fn div(self, obj: Object) -> Self::Output {
    match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Object::Integer(left / right),
      (left, right) => Error::type_mismatch("/", left, right).raise(),
    }
  }
}

impl ops::Neg for Object {
  type Output = Object;

  fn neg(self) -> Self::Output {
    match self {
      Object::Integer(number) => Object::Integer(-number),
      _ => Error::unknown_operator("-", self).raise(),
    }
  }
}

impl ops::Not for Object {
  type Output = Object;

  fn not(self) -> Self::Output {
    Object::boolean(!self.is_truthy())
  }
}

impl ops::Mul for Object {
  type Output = Object;

  fn mul(self, obj: Object) -> Self::Output {
    match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Object::Integer(left * right),
      (left, right) => Error::type_mismatch("/", left, right).raise(),
    }
  }
}

impl PartialOrd for Object {
  fn partial_cmp(&self, obj: &Object) -> Option<std::cmp::Ordering> {
    match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Some(compare(left, right)),
      (Object::String(left), Object::String(right)) => Some(compare(left, right)),
      (left, right) => Error::type_mismatch("< or >", left.clone(), right.clone()).raise(),
    }
  }
}

fn compare<T: PartialEq + PartialOrd>(left: T, right: T) -> std::cmp::Ordering {
  if left == right {
    std::cmp::Ordering::Equal
  } else if left > right {
    std::cmp::Ordering::Greater
  } else {
    std::cmp::Ordering::Less
  }
}

impl Object {
  pub const TRUE: Self = Self::Boolean(true);
  pub const FALSE: Self = Self::Boolean(false);
  pub const NULL: Self = Self::Null;

  pub fn boolean(boolean: bool) -> Self {
    if boolean {
      Self::TRUE
    } else {
      Self::FALSE
    }
  }

  pub fn r#return(obj: Self) -> Self {
    Self::Return(Box::new(obj))
  }

  pub fn is_truthy(&self) -> bool {
    match self {
      Self::Boolean(b) => *b,
      Self::Null => false,
      _ => true,
    }
  }
}
