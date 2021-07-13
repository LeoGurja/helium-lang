use crate::ast::Statement;
use crate::env::Env;
use crate::error::Error;
use crate::helpers::comma_separated;
use crate::token::Operator;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::ops;
use std::rc::Rc;

type Result = std::result::Result<Object, Error>;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
  Array(Vec<Object>),
  Integer(i64),
  String(String),
  Boolean(bool),
  Return(Box<Object>),
  Function(Vec<String>, Statement, Rc<RefCell<Env>>),
  BuiltIn(fn(Vec<Object>) -> Result),
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

impl ops::Add for Object {
  type Output = Object;

  fn add(self, obj: Object) -> Self::Output {
    match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Object::Integer(left + right),
      (Object::String(left), Object::String(right)) => Object::String(left.clone() + &right),
      (left, right) => panic!(
        "{}",
        Error::TypeMismatch(String::from("+"), left.clone(), right.clone(),)
      ),
    }
  }
}

impl ops::Sub for Object {
  type Output = Object;

  fn sub(self, obj: Object) -> Self::Output {
    match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Object::Integer(left - right),
      (left, right) => panic!("{}", Error::TypeMismatch(String::from("*"), left, right)),
    }
  }
}

impl ops::Index<Object> for Object {
  type Output = Object;

  fn index(&self, index: Object) -> &Self::Output {
    match (self, index) {
      (Object::Array(arr), Object::Integer(idx)) => arr.get(idx as usize).unwrap_or(&Object::Null),
      (Object::Hash(hash), Object::String(string)) => hash.get(&string).unwrap_or(&Object::Null),
      (left, right) => panic!("{}", Error::IndexError(left.clone(), right)),
    }
  }
}

impl ops::Div for Object {
  type Output = Object;

  fn div(self, obj: Object) -> Self::Output {
    match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Object::Integer(left / right),
      (left, right) => panic!("{}", Error::TypeMismatch(String::from("/"), left, right)),
    }
  }
}

impl ops::Neg for Object {
  type Output = Object;

  fn neg(self) -> Self::Output {
    match self {
      Object::Integer(number) => Object::Integer(-number),
      _ => panic!("{}", Error::UnknownOperator(Operator::Minus, self)),
    }
  }
}

impl ops::Not for Object {
  type Output = Object;

  fn not(self) -> Self::Output {
    Object::from(!self.is_truthy())
  }
}

impl ops::Mul for Object {
  type Output = Object;

  fn mul(self, obj: Object) -> Self::Output {
    match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Object::Integer(left * right),
      (left, right) => panic!(
        "{}",
        Error::TypeMismatch(String::from("/"), left.clone(), right.clone(),)
      ),
    }
  }
}

impl PartialOrd for Object {
  fn partial_cmp(&self, obj: &Object) -> Option<std::cmp::Ordering> {
    match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Some(compare(left, right)),
      (Object::String(left), Object::String(right)) => Some(compare(left, right)),
      (left, right) => panic!(
        "{}",
        Error::TypeMismatch(String::from("'<' or '>'"), left.clone(), right.clone())
      ),
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
  pub const TRUE: Object = Object::Boolean(true);
  pub const FALSE: Object = Object::Boolean(false);
  pub const NULL: Object = Object::Null;

  pub fn from(boolean: bool) -> Object {
    if boolean {
      Object::TRUE
    } else {
      Object::FALSE
    }
  }

  pub fn is_truthy(&self) -> bool {
    match self {
      Object::Boolean(b) => *b,
      Object::Null => false,
      _ => true,
    }
  }
}
