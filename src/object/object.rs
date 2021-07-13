use super::Type;
use crate::errors::EvalError;
use std::fmt;

type Result = std::result::Result<Object, EvalError>;

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
  pub content: Type,
}

impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.content)
  }
}

impl Object {
  pub const TRUE: Object = Object::new(Type::Boolean(true));
  pub const FALSE: Object = Object::new(Type::Boolean(false));
  pub const NULL: Object = Object::new(Type::Null);

  pub const fn new(content: Type) -> Self {
    Object { content }
  }

  pub fn not(&self) -> Object {
    match self.content {
      Type::Boolean(true) => Object::FALSE,
      Type::Boolean(false) | Type::Null => Object::TRUE,
      _ => Object::FALSE,
    }
  }

  pub fn negative(&self) -> Result {
    match self.content {
      Type::Integer(value) => Ok(Object::new(Type::Integer(-value))),
      _ => Err(EvalError::UnknownOperator(
        Box::new(String::from("-")),
        self.clone(),
      )),
    }
  }

  pub fn add(&self, obj: Object) -> Result {
    Ok(match (&self.content, &obj.content) {
      (Type::Integer(left), Type::Integer(right)) => Object::new(Type::Integer(left + right)),
      (Type::String(left), Type::String(right)) => Object::new(Type::String(left.clone() + right)),
      (left, right) => {
        return Err(EvalError::TypeMismatch(
          String::from("+"),
          left.clone(),
          right.clone(),
        ))
      }
    })
  }

  pub fn subtract(&self, obj: Object) -> Result {
    Ok(match (&self.content, &obj.content) {
      (Type::Integer(left), Type::Integer(right)) => Object::new(Type::Integer(left - right)),
      (left, right) => {
        return Err(EvalError::TypeMismatch(
          String::from("*"),
          left.clone(),
          right.clone(),
        ))
      }
    })
  }

  pub fn multiply(&self, obj: Object) -> Result {
    Ok(match (&self.content, &obj.content) {
      (Type::Integer(left), Type::Integer(right)) => Object::new(Type::Integer(left * right)),
      (left, right) => {
        return Err(EvalError::TypeMismatch(
          String::from("/"),
          left.clone(),
          right.clone(),
        ))
      }
    })
  }

  pub fn divide(&self, obj: Object) -> Result {
    Ok(match (&self.content, &obj.content) {
      (Type::Integer(left), Type::Integer(right)) => Object::new(Type::Integer(left / right)),
      (left, right) => {
        return Err(EvalError::TypeMismatch(
          String::from("/"),
          left.clone(),
          right.clone(),
        ))
      }
    })
  }

  pub fn greater_than(&self, obj: Object) -> Result {
    Ok(match (&self.content, &obj.content) {
      (Type::Integer(left), Type::Integer(right)) => {
        if left > right {
          Object::TRUE
        } else {
          Object::FALSE
        }
      }
      (left, right) => {
        return Err(EvalError::TypeMismatch(
          String::from(">"),
          left.clone(),
          right.clone(),
        ))
      }
    })
  }

  pub fn less_than(&self, obj: Object) -> Result {
    Ok(match (&self.content, &obj.content) {
      (Type::Integer(left), Type::Integer(right)) => {
        if left < right {
          Object::TRUE
        } else {
          Object::FALSE
        }
      }
      (left, right) => {
        return Err(EvalError::TypeMismatch(
          String::from("<"),
          left.clone(),
          right.clone(),
        ))
      }
    })
  }

  pub fn is_truthy(&self) -> bool {
    !self.is_falsy()
  }

  pub fn is_falsy(&self) -> bool {
    self == &Object::NULL || self == &Object::FALSE
  }
}
