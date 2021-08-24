use super::Object;
use crate::error::Error;

impl Object {
  pub fn add(&self, obj: Object) -> Result<Self, Error> {
    Ok(match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Object::Integer(left + right),
      (Object::String(left), Object::String(right)) => Object::String(format!("{}{}", left, right)),
      (left, right) => return Err(Error::type_mismatch("+", left.clone(), right)),
    })
  }

  pub fn divide(&self, obj: Object) -> Result<Self, Error> {
    match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Ok(Object::Integer(left / right)),
      (left, right) => Err(Error::type_mismatch("/", left.clone(), right)),
    }
  }

  pub fn multiply(&self, obj: Object) -> Result<Self, Error> {
    match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Ok(Object::Integer(left * right)),
      (left, right) => Err(Error::type_mismatch("/", left.clone(), right)),
    }
  }

  pub fn negate(&self) -> Result<Self, Error> {
    match self {
      Self::Integer(number) => Ok(Self::Integer(-number)),
      _ => Err(Error::unknown_operator("-".to_owned(), self.clone())),
    }
  }

  pub fn not(&self) -> Self {
    Object::boolean(!self.is_truthy())
  }

  pub fn subtract(&self, obj: Object) -> Result<Self, Error> {
    match (self, obj) {
      (Object::Integer(left), Object::Integer(right)) => Ok(Object::Integer(left - right)),
      (left, right) => Err(Error::type_mismatch("-", left.clone(), right)),
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
