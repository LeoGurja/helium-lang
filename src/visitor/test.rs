use super::*;
use crate::{
  ast::{Expression, Statement},
  object::Object,
};
use std::{collections::HashMap, rc::Rc};

#[test]
fn visit_hash_index() {
  let input = vec![
    Statement::VariableDeclaration(
      "x".to_owned(),
      Expression::Hash(vec![(
        Expression::String("leonardo".to_owned()),
        Expression::String("gurgel".to_owned()),
      )]),
    ),
    Statement::Expression(Expression::Index(
      Box::new(Expression::Id("x".to_owned())),
      Box::new(Expression::String("leonardo".to_owned())),
    )),
  ];

  let result = visit(input);

  assert_eq!(result, Object::String("gurgel".to_owned()))
}

#[test]
fn visit_hash() {
  let input = vec![
    Statement::VariableDeclaration(
      "x".to_owned(),
      Expression::Hash(vec![
        (
          Expression::String("leonardo".to_owned()),
          Expression::String("gurgel".to_owned()),
        ),
        (Expression::Integer(1), Expression::Integer(2)),
      ]),
    ),
    Statement::Expression(Expression::Id("x".to_owned())),
  ];

  let mut expected = HashMap::new();
  expected.insert("leonardo".to_owned(), Object::String("gurgel".to_owned()));
  expected.insert("1".to_owned(), Object::Integer(2));

  let result = visit(input);

  assert_eq!(result, Object::Hash(expected))
}

#[test]
fn visit_while_loop() {
  let input = vec![
    Statement::VariableDeclaration("x".to_owned(), Expression::Integer(0)),
    Statement::while_loop(
      Expression::Infix(
        "<".to_owned(),
        Box::new(Expression::Id("x".to_owned())),
        Box::new(Expression::Integer(10)),
      ),
      Statement::Expression(Expression::IdAssignment(
        "x".to_owned(),
        Box::new(Expression::Infix(
          "+".to_owned(),
          Box::new(Expression::Id("x".to_owned())),
          Box::new(Expression::Integer(1)),
        )),
      )),
    ),
    Statement::Expression(Expression::Id("x".to_owned())),
  ];

  let result = visit(input);

  assert_eq!(result, Object::Integer(10))
}

#[test]
fn visit_for_loop() {
  let input = vec![
    Statement::VariableDeclaration("x".to_owned(), Expression::Integer(0)),
    Statement::for_loop(
      "i",
      Expression::Array(vec![
        Expression::Integer(1),
        Expression::Integer(2),
        Expression::Integer(3),
      ]),
      Statement::Expression(Expression::IdAssignment(
        "x".to_owned(),
        Box::new(Expression::Id("i".to_owned())),
      )),
    ),
    Statement::Expression(Expression::Id("x".to_owned())),
  ];

  let result = visit(input);

  assert_eq!(result, Object::Integer(3))
}

#[test]
fn visit_reassign() {
  let input = vec![
    Statement::VariableDeclaration("x".to_owned(), Expression::Integer(0)),
    Statement::Expression(Expression::IdAssignment(
      "x".to_owned(),
      Box::new(Expression::Integer(1)),
    )),
    Statement::Expression(Expression::Id("x".to_owned())),
  ];

  let result = visit(input);

  assert_eq!(result, Object::Integer(1))
}

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

  assert_eq!(result, Object::Null)
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

  assert_eq!(result, Object::Integer(2))
}

#[test]
fn visit_array() {
  let input = vec![Statement::Expression(Expression::Array(vec![
    Expression::String("x".to_owned()),
    Expression::Integer(1),
  ]))];

  let result = visit(input);
  assert_eq!(
    result,
    Object::Array(vec![Object::String("x".to_owned()), Object::Integer(1)])
  )
}

#[test]
fn visit_string_concat() {
  let input = vec![Statement::Expression(Expression::Infix(
    "+".to_owned(),
    Box::new(Expression::String("leonardo".to_owned())),
    Box::new(Expression::String(" gurgel".to_owned())),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::String("leonardo gurgel".to_owned()))
}

#[test]
fn visit_string() {
  let input = vec![Statement::Expression(Expression::String(
    "leonardo gurgel".to_owned(),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::String("leonardo gurgel".to_owned()))
}

#[test]
fn visit_function_doesnt_have_frozen_parent() {
  let input = vec![
    Statement::Expression(Expression::Function(
      Some("print_i".to_owned()),
      vec![],
      Rc::new(Statement::Expression(Expression::Id("i".to_owned()))),
    )),
    Statement::VariableDeclaration("i".to_owned(), Expression::Integer(5)),
    Statement::Expression(Expression::Call(
      Box::new(Expression::Id("print_i".to_owned())),
      vec![],
    )),
  ];

  let result = visit(input);

  assert_eq!(result, Object::Integer(5))
}

#[test]
fn visit_closure() {
  let input = vec![
    Statement::Expression(Expression::Function(
      Some("adder".to_owned()),
      vec!["x".to_owned()],
      Rc::new(Statement::Expression(Expression::Function(
        None,
        vec!["y".to_owned()],
        Rc::new(Statement::Expression(Expression::Infix(
          "+".to_owned(),
          Box::new(Expression::Id("x".to_owned())),
          Box::new(Expression::Id("y".to_owned())),
        ))),
      ))),
    )),
    Statement::VariableDeclaration(
      "add_two".to_owned(),
      Expression::Call(
        Box::new(Expression::Id("adder".to_owned())),
        vec![Expression::Integer(2)],
      ),
    ),
    Statement::Expression(Expression::Call(
      Box::new(Expression::Id("add_two".to_owned())),
      vec![Expression::Integer(3)],
    )),
  ];

  let result = visit(input);

  assert_eq!(result, Object::Integer(5))
}

#[test]
fn visit_function_with_outer_scope() {
  let input = vec![
    Statement::VariableDeclaration("i".to_owned(), Expression::Integer(5)),
    Statement::Expression(Expression::Function(
      Some("print_i".to_owned()),
      vec![],
      Rc::new(Statement::Expression(Expression::Id("i".to_owned()))),
    )),
    Statement::Expression(Expression::Call(
      Box::new(Expression::Id("print_i".to_owned())),
      vec![],
    )),
  ];

  let result = visit(input);

  assert_eq!(result, Object::Integer(5))
}

#[test]
fn visit_function_call() {
  let input = vec![
    Statement::Expression(Expression::Function(
      Some("identity".to_owned()),
      vec!["x".to_owned()],
      Rc::new(Statement::Expression(Expression::Id("x".to_owned()))),
    )),
    Statement::Expression(Expression::Call(
      Box::new(Expression::Id("identity".to_owned())),
      vec![Expression::Integer(1)],
    )),
  ];

  let result = visit(input);
  assert_eq!(result, Object::Integer(1))
}

#[test]
fn visit_function_declaration() {
  let input = vec![Statement::Expression(Expression::Function(
    Some("name".to_owned()),
    vec!["argv".to_owned()],
    Rc::new(Statement::Expression(Expression::True)),
  ))];

  let result = visit(input);
  match result {
    Object::Function(args, block, ..) => {
      assert_eq!(block, Rc::new(Statement::Expression(Expression::True)));
      assert_eq!(args, vec![("argv")]);
    }
    _ => panic!("not a Function"),
  }
}

#[test]
fn visit_integer_variable_declaration() {
  let input = vec![
    Statement::VariableDeclaration("x".to_owned(), Expression::Integer(5)),
    Statement::Expression(Expression::Id("x".to_owned())),
  ];

  let result = visit(input);

  assert_eq!(result, Object::Integer(5))
}

#[test]
fn visit_boolean_variable_declaration() {
  let input = vec![
    Statement::VariableDeclaration("x".to_owned(), Expression::True),
    Statement::Expression(Expression::Id("x".to_owned())),
  ];

  let result = visit(input);

  assert_eq!(result, Object::True)
}

#[test]
fn visit_return() {
  let input = vec![
    Statement::Return(Expression::Integer(5)),
    Statement::Expression(Expression::False),
  ];

  let result = visit(input);

  assert_eq!(result, Object::Integer(5))
}

#[test]
fn visit_empty_return() {
  let input = vec![
    Statement::Return(Expression::Null),
    Statement::Expression(Expression::True),
  ];

  let result = visit(input);

  assert_eq!(result, Object::Null)
}

#[test]
fn visit_if() {
  let input = vec![Statement::Expression(Expression::Conditional(
    Box::new(Expression::True),
    Box::new(Statement::Expression(Expression::Integer(5))),
    None,
  ))];

  let result = visit(input);

  assert_eq!(result, Object::Integer(5))
}

#[test]
fn visit_else() {
  let input = vec![Statement::Expression(Expression::Conditional(
    Box::new(Expression::False),
    Box::new(Statement::Expression(Expression::Integer(5))),
    Some(Box::new(Statement::Expression(Expression::Integer(1)))),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::Integer(1))
}

#[test]
fn visit_no_else() {
  let input = vec![Statement::Expression(Expression::Conditional(
    Box::new(Expression::False),
    Box::new(Statement::Expression(Expression::Integer(5))),
    None,
  ))];

  let result = visit(input);

  assert_eq!(result, Object::Null)
}

#[test]
fn visit_infix_plus() {
  let input = vec![Statement::Expression(Expression::Infix(
    "+".to_owned(),
    Box::new(Expression::Integer(5)),
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::Integer(10))
}

#[test]
fn visit_infix_greater_than() {
  let input = vec![Statement::Expression(Expression::Infix(
    ">".to_owned(),
    Box::new(Expression::Integer(5)),
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::False)
}

#[test]
fn visit_infix_less_than() {
  let input = vec![Statement::Expression(Expression::Infix(
    "<".to_owned(),
    Box::new(Expression::Integer(1)),
    Box::new(Expression::Integer(2)),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::True)
}

#[test]
fn visit_infix_equals_on_integer() {
  let input = vec![Statement::Expression(Expression::Infix(
    "==".to_owned(),
    Box::new(Expression::Integer(5)),
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::True)
}

#[test]
fn visit_infix_not_equals_on_integer() {
  let input = vec![Statement::Expression(Expression::Infix(
    "!=".to_owned(),
    Box::new(Expression::Integer(5)),
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::False)
}

#[test]
fn visit_infix_not_equals_on_boolean() {
  let input = vec![Statement::Expression(Expression::Infix(
    "!=".to_owned(),
    Box::new(Expression::True),
    Box::new(Expression::False),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::True)
}

#[test]
fn visit_infix_equals_on_boolean() {
  let input = vec![Statement::Expression(Expression::Infix(
    "==".to_owned(),
    Box::new(Expression::True),
    Box::new(Expression::False),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::False)
}

#[test]
fn visit_infix_minus() {
  let input = vec![Statement::Expression(Expression::Infix(
    "-".to_owned(),
    Box::new(Expression::Integer(5)),
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::Integer(0))
}

#[test]
fn visit_infix_multiply() {
  let input = vec![Statement::Expression(Expression::Infix(
    "*".to_owned(),
    Box::new(Expression::Integer(5)),
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::Integer(25))
}

#[test]
fn visit_infix_divide() {
  let input = vec![Statement::Expression(Expression::Infix(
    "/".to_owned(),
    Box::new(Expression::Integer(10)),
    Box::new(Expression::Integer(2)),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::Integer(5))
}

#[test]
fn visit_minus() {
  let input = vec![Statement::Expression(Expression::Prefix(
    "-".to_owned(),
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);

  assert_eq!(result, Object::Integer(-5))
}

#[test]
fn visit_bang_on_boolean() {
  let input = vec![Statement::Expression(Expression::Prefix(
    "!".to_owned(),
    Box::new(Expression::True),
  ))];

  let result = visit(input);
  assert_eq!(result, Object::False)
}

#[test]
fn visit_bang_on_integer() {
  let input = vec![Statement::Expression(Expression::Prefix(
    "!".to_owned(),
    Box::new(Expression::Integer(5)),
  ))];

  let result = visit(input);
  assert_eq!(result, Object::False)
}

#[test]
fn visit_multiple_bangs() {
  let input = vec![Statement::Expression(Expression::Prefix(
    "!".to_owned(),
    Box::new(Expression::Prefix(
      "!".to_owned(),
      Box::new(Expression::True),
    )),
  ))];

  let result = visit(input);
  assert_eq!(result, Object::True)
}

#[test]
fn visit_integer_expression() {
  let input = vec![Statement::Expression(Expression::Integer(5))];
  let result = visit(input);
  assert_eq!(result, Object::Integer(5))
}

#[test]
fn visit_boolean_expression() {
  let input = vec![Statement::Expression(Expression::True)];

  let result = visit(input);
  assert_eq!(result, Object::True)
}

fn visit(input: Vec<Statement>) -> Object {
  let visitor = Visitor::new();
  visitor.visit(&input).unwrap()
}
