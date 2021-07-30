use super::Object;
use crate::{
  ast::{Expression, Statement},
  env,
};
use std::{collections::HashMap, rc::Rc};

#[test]
fn print_hash() {
  let mut hash = HashMap::new();
  hash.insert("name".to_owned(), Object::String("leonardo".to_owned()));
  hash.insert("last_name".to_owned(), Object::String("gurgel".to_owned()));
  let obj = Object::Hash(hash.clone());
  assert_eq!(obj.to_string(), format!("{:?}", hash))
}

#[test]
fn print_array() {
  let obj = Object::Array(vec![
    Object::Integer(1),
    Object::Integer(2),
    Object::String("leo".to_owned()),
  ]);
  assert_eq!(obj.to_string(), "[1, 2, 'leo']".to_owned())
}

#[test]
fn print_integer() {
  assert_eq!(Object::Integer(1).to_string(), "1".to_owned())
}

#[test]
fn print_string() {
  assert_eq!(
    Object::String("leonardo".to_owned()).to_string(),
    "'leonardo'".to_owned()
  )
}

#[test]
fn print_boolean() {
  assert_eq!(Object::TRUE.to_string(), "true".to_owned())
}

#[test]
fn print_return() {
  assert_eq!(
    Object::r#return(Object::String("leonardo".to_owned())).to_string(),
    "'leonardo'".to_owned()
  )
}

#[test]
fn print_function() {
  assert_eq!(
    Object::Function(
      vec!["argc".to_owned(), "argv".to_owned()],
      Rc::new(Statement::Expression(Expression::Integer(0))),
      env::global()
    )
    .to_string(),
    "fn(argc, argv)".to_owned()
  )
}

#[test]
fn print_builtin() {
  assert_eq!(
    Object::BuiltIn(|_v| Ok(Object::Null)).to_string(),
    "builtin fn()".to_owned()
  )
}

#[test]
fn print_null() {
  assert_eq!(Object::Null.to_string(), "null".to_owned())
}
