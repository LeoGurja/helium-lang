use super::parser::*;
use crate::ast::{Expression, Statement};
use crate::errors::ParserError;
use crate::lexer::Lexer;
use crate::token::Operator;

#[test]
fn reassign() {
  let input = String::from("let x = 0; x = x + 1");

  let program = parse(input);

  let expected = vec![
    Statement::Let(String::from("x"), Expression::Integer(0)),
    Statement::Expression(Expression::Infix(
      Operator::Assign,
      Box::new(Expression::Id(String::from("x"))),
      Box::new(Expression::Infix(
        Operator::Plus,
        Box::new(Expression::Id(String::from("x"))),
        Box::new(Expression::Integer(1)),
      )),
    )),
  ];

  compare(program, expected)
}

#[test]
fn while_statements() {
  let input = String::from("let x = 0; while x < 10 x = x + 1");

  let program = parse(input);

  let expected = vec![
    Statement::Let(String::from("x"), Expression::Integer(0)),
    Statement::While(
      Expression::Infix(
        Operator::LessThan,
        Box::new(Expression::Id(String::from("x"))),
        Box::new(Expression::Integer(10)),
      ),
      Box::new(Statement::Expression(Expression::Infix(
        Operator::Assign,
        Box::new(Expression::Id(String::from("x"))),
        Box::new(Expression::Infix(
          Operator::Plus,
          Box::new(Expression::Id(String::from("x"))),
          Box::new(Expression::Integer(1)),
        )),
      ))),
    ),
  ];

  compare(program, expected)
}

#[test]
fn while_blocks() {
  let input = String::from("let x = 0; while x < 10 { x = x + 1 }");

  let program = parse(input);

  let expected = vec![
    Statement::Let(String::from("x"), Expression::Integer(0)),
    Statement::While(
      Expression::Infix(
        Operator::LessThan,
        Box::new(Expression::Id(String::from("x"))),
        Box::new(Expression::Integer(10)),
      ),
      Box::new(Statement::Block(vec![Statement::Expression(
        Expression::Infix(
          Operator::Assign,
          Box::new(Expression::Id(String::from("x"))),
          Box::new(Expression::Infix(
            Operator::Plus,
            Box::new(Expression::Id(String::from("x"))),
            Box::new(Expression::Integer(1)),
          )),
        ),
      )])),
    ),
  ];

  compare(program, expected)
}

#[test]
fn for_statements() {
  let input = String::from("for a in [1,2,3] let x = a + 1");

  let program = parse(input);

  let expected = vec![Statement::For(
    String::from("a"),
    Expression::Array(vec![
      Expression::Integer(1),
      Expression::Integer(2),
      Expression::Integer(3),
    ]),
    Box::new(Statement::Let(
      String::from("x"),
      Expression::Infix(
        Operator::Plus,
        Box::new(Expression::Id(String::from("a"))),
        Box::new(Expression::Integer(1)),
      ),
    )),
  )];

  compare(program, expected)
}

#[test]
fn for_blocks() {
  let input = String::from("for a in [1,2,3] { let x = a + 1 }");

  let program = parse(input);

  let expected = vec![Statement::For(
    String::from("a"),
    Expression::Array(vec![
      Expression::Integer(1),
      Expression::Integer(2),
      Expression::Integer(3),
    ]),
    Box::new(Statement::Block(vec![Statement::Let(
      String::from("x"),
      Expression::Infix(
        Operator::Plus,
        Box::new(Expression::Id(String::from("a"))),
        Box::new(Expression::Integer(1)),
      ),
    )])),
  )];

  compare(program, expected)
}

#[test]
fn array_expressions() {
  let input = String::from("myArray[1 + 1]");

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::Index(
    Box::new(Expression::Id(String::from("myArray"))),
    Box::new(Expression::Infix(
      Operator::Plus,
      Box::new(Expression::Integer(1)),
      Box::new(Expression::Integer(1)),
    )),
  ))];

  compare(program, expected)
}

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
      Operator::Plus,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      Operator::Minus,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      Operator::Asterisk,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      Operator::Slash,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      Operator::GreaterThan,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      Operator::LessThan,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      Operator::Equals,
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      Operator::NotEquals,
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
      Operator::Bang,
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Prefix(
      Operator::Minus,
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
fn function_declarations_with_args() {
  let input = String::from("fn add(a, b) a + b");

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::Function(
    Some(String::from("add")),
    vec![String::from("a"), String::from("b")],
    Box::new(Statement::Expression(Expression::Infix(
      Operator::Plus,
      Box::new(Expression::Id(String::from("a"))),
      Box::new(Expression::Id(String::from("b"))),
    ))),
  ))];

  compare(program, expected)
}

#[test]
fn function_blocks_with_args() {
  let input = String::from(
    "fn add(a, b) {
      a + b
    }",
  );

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::Function(
    Some(String::from("add")),
    vec![String::from("a"), String::from("b")],
    Box::new(Statement::Block(vec![Statement::Expression(
      Expression::Infix(
        Operator::Plus,
        Box::new(Expression::Id(String::from("a"))),
        Box::new(Expression::Id(String::from("b"))),
      ),
    )])),
  ))];

  compare(program, expected)
}

#[test]
fn function_declarations() {
  let input = String::from("fn main() 0");

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::Function(
    Some(String::from("main")),
    vec![],
    Box::new(Statement::Expression(Expression::Integer(0))),
  ))];

  compare(program, expected)
}

#[test]
fn function_blocks() {
  let input = String::from(
    "fn main() {
      0
    }",
  );

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::Function(
    Some(String::from("main")),
    vec![],
    Box::new(Statement::Block(vec![Statement::Expression(
      Expression::Integer(0),
    )])),
  ))];

  compare(program, expected)
}

#[test]
fn call_expressions() {
  let input = String::from("add(3, 5);");

  let program = parse(input);
  let expected = vec![Statement::Expression(Expression::Call(
    Box::new(Expression::Id(String::from("add"))),
    vec![Expression::Integer(3), Expression::Integer(5)],
  ))];

  compare(program, expected)
}

#[test]
fn if_expressions() {
  let input = String::from(
    "if x > y return x; else return y;
    let result = if (x > y) x else y",
  );

  let expected = vec![
    Statement::Expression(Expression::If(
      Box::new(Expression::Infix(
        Operator::GreaterThan,
        Box::new(Expression::Id(String::from("x"))),
        Box::new(Expression::Id(String::from("y"))),
      )),
      Box::new(Statement::Return(Some(Expression::Id(String::from("x"))))),
      Some(Box::new(Statement::Return(Some(Expression::Id(
        String::from("y"),
      ))))),
    )),
    Statement::Let(
      String::from("result"),
      Expression::If(
        Box::new(Expression::Infix(
          Operator::GreaterThan,
          Box::new(Expression::Id(String::from("x"))),
          Box::new(Expression::Id(String::from("y"))),
        )),
        Box::new(Statement::Expression(Expression::Id(String::from("x")))),
        Some(Box::new(Statement::Expression(Expression::Id(
          String::from("y"),
        )))),
      ),
    ),
  ];

  compare(parse(input), expected)
}

#[test]
fn if_block() {
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
        Operator::GreaterThan,
        Box::new(Expression::Id(String::from("x"))),
        Box::new(Expression::Id(String::from("y"))),
      )),
      Box::new(Statement::Block(vec![Statement::Return(Some(
        Expression::Id(String::from("x")),
      ))])),
      Some(Box::new(Statement::Block(vec![Statement::Return(Some(
        Expression::Id(String::from("y")),
      ))]))),
    )),
    Statement::Let(
      String::from("result"),
      Expression::If(
        Box::new(Expression::Infix(
          Operator::GreaterThan,
          Box::new(Expression::Id(String::from("x"))),
          Box::new(Expression::Id(String::from("y"))),
        )),
        Box::new(Statement::Block(vec![Statement::Expression(
          Expression::Id(String::from("x")),
        )])),
        Some(Box::new(Statement::Block(vec![Statement::Expression(
          Expression::Id(String::from("y")),
        )]))),
      ),
    ),
  ];

  compare(parse(input), expected)
}

#[test]
fn semicolons() {
  compare(
    parse(String::from("1 + 1;")),
    to_block(parse(String::from("1 + 1"))),
  )
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
    ("!-c", "(!(-c))"),
    ("d + e + f", "((d + e) + f)"),
    ("g + h - i", "((g + h) - i)"),
    ("j * k * l", "((j * k) * l)"),
    ("m * n / o", "((m * n) / o)"),
    ("p + q / r", "(p + (q / r))"),
    ("s + t * u + v / w - x", "(((s + (t * u)) + (v / w)) - x)"),
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
    compare(
      parse(String::from(actual)),
      to_block(parse(String::from(expected))),
    );
  }
}

fn parse(input: String) -> Result<Vec<Statement>, Vec<ParserError>> {
  let mut parser = Parser::new(Lexer::new(input));
  parser.parse()
}

fn compare(program: Result<Vec<Statement>, Vec<ParserError>>, expected: Vec<Statement>) {
  for (a, b) in to_block(program).iter().zip(expected.iter()) {
    assert_eq!(a, b)
  }
}

fn to_block(program: Result<Vec<Statement>, Vec<ParserError>>) -> Vec<Statement> {
  match program {
    Ok(p) => p,
    Err(e) => panic!("Parser has errors:\n\t{:?}", e),
  }
}
