use super::parser::*;
use crate::ast::{Block, Expression, Infix, Prefix, Statement};
use crate::lexer::Lexer;

#[test]
fn escaped_string_expressions() {
  let input = String::from(r#""leonardo \"gurgel""#);

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::String(String::from(
    r#"leonardo "gurgel"#,
  )))];

  compare(program, expected)
}

#[test]
fn string_expressions() {
  let input = String::from("\"leonardo gurgel\"");

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::String(String::from(
    "leonardo gurgel",
  )))];

  compare(program, expected)
}

#[test]
fn infix_expressions() {
  let input = String::from(
    "
    5 + 5;
    5 - 5;
    5 * 5;
    5 / 5;
    5 > 5;
    5 < 5;
    5 == 5;
    5 != 5;
    ",
  );

  let program = parse(input);

  let expected = vec![
    Statement::Expression(Expression::Infix(
      Infix::Plus,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      Infix::Minus,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      Infix::Asterisk,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      Infix::Slash,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      Infix::GreaterThan,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      Infix::LessThan,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      Infix::Equals,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      Infix::NotEquals,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
  ];

  compare(program, expected)
}

#[test]
fn prefix_expressions() {
  let input = String::from(
    "!5;
  -15;",
  );

  let program = parse(input);

  let expected = vec![
    Statement::Expression(Expression::Prefix(
      Prefix::Bang,
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Prefix(
      Prefix::Minus,
      Box::new(Expression::Integer(15)),
    )),
  ];

  compare(program, expected)
}

#[test]
fn return_statements() {
  let input = String::from(
    "return 5;
    return 10;
    return 993322;",
  );

  let program = parse(input);

  let expected = vec![
    Statement::Return(Some(Expression::Integer(5))),
    Statement::Return(Some(Expression::Integer(10))),
    Statement::Return(Some(Expression::Integer(993322))),
  ];

  compare(program, expected)
}

#[test]
fn let_statements() {
  let input = String::from(
    "
  let x = 5;
  let y = 10;
  let foobar = 838383;
  ",
  );

  let program = parse(input);

  let expected = vec![
    Statement::Let(String::from("x"), Expression::Integer(5)),
    Statement::Let(String::from("y"), Expression::Integer(10)),
    Statement::Let(String::from("foobar"), Expression::Integer(838383)),
  ];

  compare(program, expected)
}

#[test]
fn function_declarations() {
  let input = String::from(
    "fn main() {
      0
    }",
  );

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::Function(
    Some(String::from("main")),
    vec![],
    vec![Statement::Expression(Expression::Integer(0))],
  ))];

  compare(program, expected)
}

#[test]
fn call_expressions() {
  let input = String::from("add(3, 5);");

  let program = parse(input);
  let expected = vec![Statement::Expression(Expression::Call(
    String::from("add"),
    vec![Expression::Integer(3), Expression::Integer(5)],
  ))];

  compare(program, expected)
}

#[test]
fn if_expressions() {
  let input = String::from(
    "if x > y {
      return x;
    } else {
      return y;
    }
    let result = if (x > y) { x } else { y }",
  );

  let expected = vec![
    Statement::Expression(Expression::If(
      Box::new(Expression::Infix(
        Infix::GreaterThan,
        Box::new(Expression::Id(String::from("x"))),
        Box::new(Expression::Id(String::from("y"))),
      )),
      vec![Statement::Return(Some(Expression::Id(String::from("x"))))],
      Some(vec![Statement::Return(Some(Expression::Id(String::from(
        "y",
      ))))]),
    )),
    Statement::Let(
      String::from("result"),
      Expression::If(
        Box::new(Expression::Infix(
          Infix::GreaterThan,
          Box::new(Expression::Id(String::from("x"))),
          Box::new(Expression::Id(String::from("y"))),
        )),
        vec![Statement::Expression(Expression::Id(String::from("x")))],
        Some(vec![Statement::Expression(Expression::Id(String::from(
          "y",
        )))]),
      ),
    ),
  ];

  compare(parse(input), expected)
}

#[test]
fn semicolons() {
  compare(parse(String::from("1 + 1;")), parse(String::from("1 + 1")))
}

#[test]
fn boolean_expressions() {
  let input = String::from(
    "true;
    false;
    let foobar = true;
    let barfoo = false;",
  );

  let expected = vec![
    Statement::Expression(Expression::TRUE),
    Statement::Expression(Expression::FALSE),
    Statement::Let(String::from("foobar"), Expression::TRUE),
    Statement::Let(String::from("barfoo"), Expression::FALSE),
  ];
  compare(parse(input), expected)
}

#[test]
fn precedence() {
  let inputs = vec![
    ("-a * b", "((-a) * b)"),
    ("!-a", "(!(-a))"),
    ("a + b + c", "((a + b) + c)"),
    ("a + b - c", "((a + b) - c)"),
    ("a * b * c", "((a * b) * c)"),
    ("a * b / c", "((a * b) / c)"),
    ("a + b / c", "(a + (b / c))"),
    ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
    ("3 + 4; -5 * 5", "(3 + 4); ((-5) * 5)"),
    ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
    ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
    (
      "3 + 4 * 5 == 3 * 1 + 4 * 5",
      "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
    ),
    (
      "3 + 4 * 5 == 3 * 1 + 4 * 5",
      "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
    ),
  ];

  for (actual, expected) in inputs {
    compare(parse(String::from(actual)), parse(String::from(expected)))
  }
}

fn parse(input: String) -> Block {
  let parser = Parser::new(Lexer::new(input));
  let program = parser.parse();
  check_errors(parser);
  program
}

fn compare(program: Block, expected: Vec<Statement>) {
  for (actual, expected) in program.iter().zip(expected.iter()) {
    assert_eq!(actual, expected)
  }
}

fn check_errors(parser: Parser) {
  let errors = parser.errors.borrow();
  if errors.len() > 0 {
    panic!("Parser has {} errors: {:?}", errors.len(), errors);
  }
}
