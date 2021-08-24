use super::parser::*;
use crate::ast::{Expression, Statement};
use crate::lexer::Token;
use logos::Logos;
use std::rc::Rc;

#[test]
fn hash_indexes() {
  let input = "hash['leonardo']";

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::Index(
    Box::new(Expression::Id("hash".to_owned())),
    Box::new(Expression::String("leonardo".to_owned())),
  ))];

  compare(program, expected)
}

#[test]
fn hash() {
  let input = "let x = {'leonardo': 'gurgel', 1: 'ferreira'}";

  let program = parse(input);

  let expected = vec![Statement::VariableDeclaration(
    "x".to_owned(),
    Expression::Hash(vec![
      (
        Expression::String("leonardo".to_owned()),
        Expression::String("gurgel".to_owned()),
      ),
      (
        Expression::Integer(1),
        Expression::String("ferreira".to_owned()),
      ),
    ]),
  )];

  compare(program, expected)
}

#[test]
fn reassign() {
  let input = "let x = 0; x = x + 1";

  let program = parse(input);

  let expected = vec![
    Statement::VariableDeclaration("x".to_owned(), Expression::Integer(0)),
    Statement::Expression(Expression::IdAssignment(
      "x".to_owned(),
      Box::new(Expression::Infix(
        "+".to_owned(),
        Box::new(Expression::Id("x".to_owned())),
        Box::new(Expression::Integer(1)),
      )),
    )),
  ];

  compare(program, expected)
}

#[test]
fn while_statements() {
  let input = "let x = 0; while x < 10 x = x + 1";

  let program = parse(input);

  let expected = vec![
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
  ];

  compare(program, expected)
}

#[test]
fn while_blocks() {
  let input = "let x = 0; while x < 10 { x = x + 1 }";

  let program = parse(input);

  let expected = vec![
    Statement::VariableDeclaration("x".to_owned(), Expression::Integer(0)),
    Statement::while_loop(
      Expression::Infix(
        "<".to_owned(),
        Box::new(Expression::Id("x".to_owned())),
        Box::new(Expression::Integer(10)),
      ),
      Statement::Block(vec![Statement::Expression(Expression::IdAssignment(
        "x".to_owned(),
        Box::new(Expression::Infix(
          "+".to_owned(),
          Box::new(Expression::Id("x".to_owned())),
          Box::new(Expression::Integer(1)),
        )),
      ))]),
    ),
  ];

  compare(program, expected)
}

#[test]
fn for_statements() {
  let input = "for a in [1,2,3] let x = a + 1";

  let program = parse(input);

  let expected = vec![Statement::for_loop(
    "a",
    Expression::Array(vec![
      Expression::Integer(1),
      Expression::Integer(2),
      Expression::Integer(3),
    ]),
    Statement::VariableDeclaration(
      "x".to_owned(),
      Expression::Infix(
        "+".to_owned(),
        Box::new(Expression::Id("a".to_owned())),
        Box::new(Expression::Integer(1)),
      ),
    ),
  )];

  compare(program, expected)
}

#[test]
fn for_blocks() {
  let input = "for a in [1,2,3] { let x = a + 1 }";

  let program = parse(input);

  let expected = vec![Statement::for_loop(
    "a",
    Expression::Array(vec![
      Expression::Integer(1),
      Expression::Integer(2),
      Expression::Integer(3),
    ]),
    Statement::Block(vec![Statement::VariableDeclaration(
      "x".to_owned(),
      Expression::Infix(
        "+".to_owned(),
        Box::new(Expression::Id("a".to_owned())),
        Box::new(Expression::Integer(1)),
      ),
    )]),
  )];

  compare(program, expected)
}

#[test]
fn array_expressions() {
  let input = "myArray[1 + 1]";

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::Index(
    Box::new(Expression::Id("myArray".to_owned())),
    Box::new(Expression::Infix(
      "+".to_owned(),
      Box::new(Expression::Integer(1)),
      Box::new(Expression::Integer(1)),
    )),
  ))];

  compare(program, expected)
}

#[test]
fn escaped_string_expressions() {
  let input = r#""leonardo \"gurgel""#;

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::String(
    r#"leonardo "gurgel"#.to_owned(),
  ))];

  compare(program, expected)
}

#[test]
fn string_expressions() {
  let input = "\"leonardo gurgel\"";

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::String(
    "leonardo gurgel".to_owned(),
  ))];

  compare(program, expected)
}

#[test]
fn infix_expressions() {
  let input = "
    5 + 5;
    5 - 5;
    5 * 5;
    5 / 5;
    5 > 5;
    5 < 5;
    5 == 5;
    5 != 5;
    ";

  let program = parse(input);

  let expected: Vec<Statement> = vec![
    Statement::Expression(Expression::Infix(
      "+".to_owned(),
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      "-".to_owned(),
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      "*".to_owned(),
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      "/".to_owned(),
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      ">".to_owned(),
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      "<".to_owned(),
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      "==".to_owned(),
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Infix(
      "!=".to_owned(),
      Box::new(Expression::Integer(5)),
      Box::new(Expression::Integer(5)),
    )),
  ];

  compare(program, expected)
}

#[test]
fn prefix_expressions() {
  let input = "!5;
  -15;";

  let program = parse(input);

  let expected = vec![
    Statement::Expression(Expression::Prefix(
      "!".to_owned(),
      Box::new(Expression::Integer(5)),
    )),
    Statement::Expression(Expression::Prefix(
      "-".to_owned(),
      Box::new(Expression::Integer(15)),
    )),
  ];

  compare(program, expected)
}

#[test]
fn return_statements() {
  let input = "return 5;
    return 10;
    return 993322;
    return
    ";

  let program = parse(input);

  let expected = vec![
    Statement::Return(Expression::Integer(5)),
    Statement::Return(Expression::Integer(10)),
    Statement::Return(Expression::Integer(993322)),
    Statement::Return(Expression::Null),
  ];

  compare(program, expected)
}

#[test]
fn let_statements() {
  let input = "
  let x = 5;
  let y = 10;
  let foobar = 838383;
  ";

  let program = parse(input);

  let expected = vec![
    Statement::VariableDeclaration("x".to_owned(), Expression::Integer(5)),
    Statement::VariableDeclaration("y".to_owned(), Expression::Integer(10)),
    Statement::VariableDeclaration("foobar".to_owned(), Expression::Integer(838383)),
  ];

  compare(program, expected)
}

#[test]
fn function_declarations_with_args() {
  let input = "fn add(a, b) a + b";

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::Function(
    Some("add".to_owned()),
    vec!["a".to_owned(), "b".to_owned()],
    Rc::new(Statement::Expression(Expression::Infix(
      "+".to_owned(),
      Box::new(Expression::Id("a".to_owned())),
      Box::new(Expression::Id("b".to_owned())),
    ))),
  ))];

  compare(program, expected)
}

#[test]
fn function_blocks_with_args() {
  let input = "fn add(a, b) {
      a + b
    }";

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::Function(
    Some("add".to_owned()),
    vec!["a".to_owned(), "b".to_owned()],
    Rc::new(Statement::Block(vec![Statement::Expression(
      Expression::Infix(
        "+".to_owned(),
        Box::new(Expression::Id("a".to_owned())),
        Box::new(Expression::Id("b".to_owned())),
      ),
    )])),
  ))];

  compare(program, expected)
}

#[test]
fn function_declarations() {
  let input = "fn main() 0";

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::Function(
    Some("main".to_owned()),
    vec![],
    Rc::new(Statement::Expression(Expression::Integer(0))),
  ))];

  compare(program, expected)
}

#[test]
fn function_blocks() {
  let input = "fn main() {
      0
    }";

  let program = parse(input);

  let expected = vec![Statement::Expression(Expression::Function(
    Some("main".to_owned()),
    vec![],
    Rc::new(Statement::Block(vec![Statement::Expression(
      Expression::Integer(0),
    )])),
  ))];

  compare(program, expected)
}

#[test]
fn call_expressions() {
  let input = "add(3, 5);";

  let program = parse(input);
  let expected = vec![Statement::Expression(Expression::Call(
    Box::new(Expression::Id("add".to_owned())),
    vec![Expression::Integer(3), Expression::Integer(5)],
  ))];

  compare(program, expected)
}

#[test]
fn if_expressions() {
  let input = "if x > y return x; else return y;
    let result = if (x > y) x else y";

  let expected = vec![
    Statement::Expression(Expression::Conditional(
      Box::new(Expression::Infix(
        ">".to_owned(),
        Box::new(Expression::Id("x".to_owned())),
        Box::new(Expression::Id("y".to_owned())),
      )),
      Box::new(Statement::Return(Expression::Id("x".to_owned()))),
      Some(Box::new(Statement::Return(Expression::Id("y".to_owned())))),
    )),
    Statement::VariableDeclaration(
      "result".to_owned(),
      Expression::Conditional(
        Box::new(Expression::Infix(
          ">".to_owned(),
          Box::new(Expression::Id("x".to_owned())),
          Box::new(Expression::Id("y".to_owned())),
        )),
        Box::new(Statement::Expression(Expression::Id("x".to_owned()))),
        Some(Box::new(Statement::Expression(Expression::Id(
          "y".to_owned(),
        )))),
      ),
    ),
  ];

  compare(parse(input), expected)
}

#[test]
fn if_block() {
  let input = "if x > y {
      return x;
    } else {
      return y;
    }
    let result = if (x > y) { x } else { y }";

  let expected = vec![
    Statement::Expression(Expression::Conditional(
      Box::new(Expression::Infix(
        ">".to_owned(),
        Box::new(Expression::Id("x".to_owned())),
        Box::new(Expression::Id("y".to_owned())),
      )),
      Box::new(Statement::Block(vec![Statement::Return(Expression::Id(
        "x".to_owned(),
      ))])),
      Some(Box::new(Statement::Block(vec![Statement::Return(
        Expression::Id("y".to_owned()),
      )]))),
    )),
    Statement::VariableDeclaration(
      "result".to_owned(),
      Expression::Conditional(
        Box::new(Expression::Infix(
          ">".to_owned(),
          Box::new(Expression::Id("x".to_owned())),
          Box::new(Expression::Id("y".to_owned())),
        )),
        Box::new(Statement::Block(vec![Statement::Expression(
          Expression::Id("x".to_owned()),
        )])),
        Some(Box::new(Statement::Block(vec![Statement::Expression(
          Expression::Id("y".to_owned()),
        )]))),
      ),
    ),
  ];

  compare(parse(input), expected)
}

#[test]
fn semicolons() {
  compare(parse("1 + 1;"), parse("1 + 1"))
}

#[test]
fn boolean_expressions() {
  let input = "true;
    false;
    let foobar = true;
    let barfoo = false;";

  let expected = vec![
    Statement::Expression(Expression::True),
    Statement::Expression(Expression::False),
    Statement::VariableDeclaration("foobar".to_owned(), Expression::True),
    Statement::VariableDeclaration("barfoo".to_owned(), Expression::False),
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
    compare(parse(actual), parse(expected));
  }
}

fn parse(input: &str) -> Vec<Statement> {
  let mut parser = Parser::new(Token::lexer(input));
  let program = parser.parse();

  for err in parser.errors {
    println!("{}", err);
  }
  program
}

fn compare(program: Vec<Statement>, expected: Vec<Statement>) {
  for (a, b) in program.iter().zip(expected.iter()) {
    assert_eq!(a, b)
  }
}
