use super::*;
use crate::ast::{Expression, Statement};
use crate::object::{Object, Type};
use crate::token::Operator;

#[test]
fn visit_out_of_bounds_index() {
  let input = vec![Statement::Expression(Expression::Index(
    Box::new(Expression::Array(vec![
      Expression::Integer(1),
      Expression::Integer(2),
      Expression::Integer(3),
    ])),
    Box::new(Expression::Integer(4)),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::NULL)
}

#[test]
fn visit_array_index() {
  let input = vec![Statement::Expression(Expression::Index(
    Box::new(Expression::Array(vec![
      Expression::Integer(1),
      Expression::Integer(2),
      Expression::Integer(3),
    ])),
    Box::new(Expression::Integer(1)),
  ))];

  let result = visit(input);

  assert_eq!(result.content, Type::Integer(2))
}

#[test]
fn visit_array() {
  let input = vec![Statement::Expression(Expression::Array(vec![
    Expression::String(String::from("x")),
    Expression::Integer(1),
  ]))];

  let result = visit(input);
  assert_eq!(
    result.content,
    Type::Array(vec![
      Object::new(Type::String(String::from("x"))),
      Object::new(Type::Integer(1))
    ])
  )
}

#[test]
fn visit_string_concat() {
  let input = vec![Statement::Expression(Expression::Infix(
    Operator::Plus,
    Box::new(Expression::String(String::from("leonardo"))),
    Box::new(Expression::String(String::from(" gurgel"))),
  ))];

  let result = visit(input);

  assert_eq!(
    result.content,
    Type::String(String::from("leonardo gurgel"))
  )
}

#[test]
fn visit_string() {
  let input = vec![Statement::Expression(Expression::String(String::from(
    "leonardo gurgel",
  )))];

  let result = visit(input);

  assert_eq!(
    result.content,
    Type::String(String::from("leonardo gurgel"))
  )
}

#[test]
fn visit_function_doesnt_have_frozen_parent() {
  let input = vec![
    Statement::Expression(Expression::Function(
      Some(String::from("print_i")),
      vec![],
      Box::new(Statement::Expression(Expression::Id(String::from("i")))),
    )),
    Statement::Let(String::from("i"), Expression::Integer(5)),
    Statement::Expression(Expression::Call(
      Box::new(Expression::Id(String::from("print_i"))),
      vec![],
    )),
  ];

  let result = visit(input);

  assert_eq!(result.content, Type::Integer(5))
}

#[test]
fn visit_closure() {
  let input = vec![
    Statement::Expression(Expression::Function(
      Some(String::from("adder")),
      vec![String::from("x")],
      Box::new(Statement::Expression(Expression::Function(
        None,
        vec![String::from("y")],
        Box::new(Statement::Expression(Expression::Infix(
          Operator::Plus,
          Box::new(Expression::Id(String::from("x"))),
          Box::new(Expression::Id(String::from("y"))),
        ))),
      ))),
    )),
    Statement::Let(
      String::from("add_two"),
      Expression::Call(
        Box::new(Expression::Id(String::from("adder"))),
        vec![Expression::Integer(2)],
      ),
    ),
    Statement::Expression(Expression::Call(
      Box::new(Expression::Id(String::from("add_two"))),
      vec![Expression::Integer(3)],
    )),
  ];

  let result = visit(input);

  assert_eq!(result.content, Type::Integer(5))
}

#[test]
fn visit_function_with_outer_scope() {
  let input = vec![
    Statement::Let(String::from("i"), Expression::Integer(5)),
    Statement::Expression(Expression::Function(
      Some(String::from("print_i")),
      vec![],
      Box::new(Statement::Expression(Expression::Id(String::from("i")))),
    )),
    Statement::Expression(Expression::Call(
      Box::new(Expression::Id(String::from("print_i"))),
      vec![],
    )),
  ];

  let result = visit(input);

  assert_eq!(result.content, Type::Integer(5))
}

#[test]
fn visit_function_call() {
  let input = vec![
    Statement::Expression(Expression::Function(
      Some(String::from("identity")),
      vec![String::from("x")],
      Box::new(Statement::Expression(Expression::Id(String::from("x")))),
    )),
    Statement::Expression(Expression::Call(
      Box::new(Expression::Id(String::from("identity"))),
      vec![Expression::Integer(1)],
    )),
  ];

  let result = visit(input);
  assert_eq!(result.content, Type::Integer(1))
}

#[test]
fn visit_function_declaration() {
  let input = vec![Statement::Expression(Expression::Function(
    Some(String::from("name")),
    vec![String::from("argv")],
    Box::new(Statement::Expression(Expression::TRUE)),
  ))];

  let result = visit(input);
  match result.content {
    Type::Function(args, block, ..) => {
      assert_eq!(block, Statement::Expression(Expression::TRUE));
      assert_eq!(args, vec![String::from("argv")]);
    }
    _ => panic!("not a function"),
  }
}

#[test]
fn visit_integer_variable_declaration() {
  let input = vec![
    Statement::Let(String::from("x"), Expression::Integer(5)),
    Statement::Expression(Expression::Id(String::from("x"))),
  ];

  let result = visit(input);

  assert_eq!(result.content, Type::Integer(5))
}

#[test]
fn visit_boolean_variable_declaration() {
  let input = vec![
    Statement::Let(String::from("x"), Expression::TRUE),
    Statement::Expression(Expression::Id(String::from("x"))),
  ];

  let result = visit(input);

  assert_eq!(result, Object::TRUE)
}

#[test]
fn visit_return() {
  let input = vec![
    Statement::Return(Some(Expression::Integer(5))),
    Statement::Expression(Expression::FALSE),
  ];

  let result = visit(input);

  assert_eq!(result.content, Type::Integer(5))
}

#[test]
fn visit_empty_return() {
  let input = vec![
    Statement::Return(None),
    Statement::Expression(Expression::TRUE),
  ];

  let result = visit(input);

  assert_eq!(result, Object::NULL)
}

#[test]
fn visit_if() {
  let input = vec![Statement::Expression(Expression::If(
    Box::new(Expression::TRUE),
    Box::new(Statement::Expression(Expression::Integer(5))),
    None,
  ))];

  let result = visit(input);

  assert_eq!(result.content, Type::Integer(5))
}

#[test]
fn visit_else() {
  let input = vec![Statement::Expression(Expression::If(
    Box::new(Expression::FALSE),
    Box::new(Statement::Expression(Expression::Integer(5))),
    Some(Box::new(Statement::Expression(Expression::Integer(1)))),
  ))];

  let result = visit(input);

  assert_eq!(result.content, Type::Integer(1))
}

#[test]
fn visit_no_else() {
  let input = vec![Statement::Expression(Expression::If(
    Box::new(Expression::FALSE),
    Box::new(Statement::Expression(Expression::Integer(5))),
    None,
  ))];

  let result = visit(input);

  assert_eq!(result, Object::NULL)
}

#[test]
fn visit_infix_plus() {
  let input = vec![Statement::Expression(Expression::Infix(
    Operator::Plus,
    Box::new(Expression::Integer(5)),
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);

  assert_eq!(result.content, Type::Integer(10))
}

#[test]
fn visit_infix_greater_than() {
  let input = vec![Statement::Expression(Expression::Infix(
    Operator::GreaterThan,
    Box::new(Expression::Integer(5)),
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::FALSE)
}

#[test]
fn visit_infix_less_than() {
  let input = vec![Statement::Expression(Expression::Infix(
    Operator::LessThan,
    Box::new(Expression::Integer(1)),
    Box::new(Expression::Integer(2)),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::TRUE)
}

#[test]
fn visit_infix_equals_on_integer() {
  let input = vec![Statement::Expression(Expression::Infix(
    Operator::Equals,
    Box::new(Expression::Integer(5)),
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::TRUE)
}

#[test]
fn visit_infix_not_equals_on_integer() {
  let input = vec![Statement::Expression(Expression::Infix(
    Operator::NotEquals,
    Box::new(Expression::Integer(5)),
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::FALSE)
}

#[test]
fn visit_infix_not_equals_on_boolean() {
  let input = vec![Statement::Expression(Expression::Infix(
    Operator::NotEquals,
    Box::new(Expression::TRUE),
    Box::new(Expression::FALSE),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::TRUE)
}

#[test]
fn visit_infix_equals_on_boolean() {
  let input = vec![Statement::Expression(Expression::Infix(
    Operator::Equals,
    Box::new(Expression::TRUE),
    Box::new(Expression::FALSE),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::FALSE)
}

#[test]
fn visit_infix_minus() {
  let input = vec![Statement::Expression(Expression::Infix(
    Operator::Minus,
    Box::new(Expression::Integer(5)),
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);

  assert_eq!(result.content, Type::Integer(0))
}

#[test]
fn visit_infix_multiply() {
  let input = vec![Statement::Expression(Expression::Infix(
    Operator::Asterisk,
    Box::new(Expression::Integer(5)),
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);

  assert_eq!(result.content, Type::Integer(25))
}

#[test]
fn visit_infix_divide() {
  let input = vec![Statement::Expression(Expression::Infix(
    Operator::Slash,
    Box::new(Expression::Integer(10)),
    Box::new(Expression::Integer(2)),
  ))];

  let result = visit(input);

  assert_eq!(result.content, Type::Integer(5))
}

#[test]
fn visit_minus() {
  let input = vec![Statement::Expression(Expression::Prefix(
    Operator::Minus,
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);

  assert_eq!(result.content, Type::Integer(-5))
}

#[test]
fn visit_bang_on_boolean() {
  let input = vec![Statement::Expression(Expression::Prefix(
    Operator::Bang,
    Box::new(Expression::TRUE),
  ))];

  let result = visit(input);
  assert_eq!(result, Object::FALSE)
}

#[test]
fn visit_bang_on_integer() {
  let input = vec![Statement::Expression(Expression::Prefix(
    Operator::Bang,
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);
  assert_eq!(result, Object::FALSE)
}

#[test]
fn visit_multiple_bangs() {
  let input = vec![Statement::Expression(Expression::Prefix(
    Operator::Bang,
    Box::new(Expression::Prefix(
      Operator::Bang,
      Box::new(Expression::TRUE),
    )),
  ))];

  let result = visit(input);
  assert_eq!(result, Object::TRUE)
}

#[test]
fn visit_integer_expression() {
  let input = vec![Statement::Expression(Expression::Integer(5))];
  let result = visit(input);
  assert_eq!(result.content, Type::Integer(5))
}

#[test]
fn visit_boolean_expression() {
  let input = vec![Statement::Expression(Expression::TRUE)];

  let result = visit(input);
  assert_eq!(result, Object::new(Type::Boolean(true)))
}

fn visit(input: Vec<Statement>) -> Object {
  let visitor = Visitor::new();
  let result = visitor.visit(&input);

  match result {
    Err(err) => panic!("visitor returned error: {}", err),
    Ok(value) => value,
  }
}
