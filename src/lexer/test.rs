use super::*;
use crate::token::{Operator, Token};

#[test]
fn hash_indexes() {
  let input = String::from("hash['leonardo']");

  let expected = vec![
    Token::Id(String::from("hash")),
    Token::LeftBracket,
    Token::String(String::from("leonardo")),
    Token::RightBracket,
    Token::Eof,
  ];

  compare(input, expected)
}

#[test]
fn hashes() {
  let input = String::from("let x = {'name': 'leonardo', 'last_name': 'ferreira', 1: 1}");

  let expected = vec![
    Token::Let,
    Token::Id(String::from("x")),
    Token::Operator(Operator::Assign),
    Token::LeftBrace,
    Token::String(String::from("name")),
    Token::Colon,
    Token::String(String::from("leonardo")),
    Token::Comma,
    Token::String(String::from("last_name")),
    Token::Colon,
    Token::String(String::from("ferreira")),
    Token::Comma,
    Token::Integer(String::from("1")),
    Token::Colon,
    Token::Integer(String::from("1")),
    Token::RightBrace,
    Token::Eof,
  ];

  compare(input, expected)
}

#[test]
fn while_loops() {
  let input = String::from("let x = 0; while x < 10 { x = x + 1 }");

  let expected = vec![
    Token::Let,
    Token::Id(String::from("x")),
    Token::Operator(Operator::Assign),
    Token::Integer(String::from("0")),
    Token::Semicolon,
    Token::While,
    Token::Id(String::from("x")),
    Token::Operator(Operator::LessThan),
    Token::Integer(String::from("10")),
    Token::LeftBrace,
    Token::Id(String::from("x")),
    Token::Operator(Operator::Assign),
    Token::Id(String::from("x")),
    Token::Operator(Operator::Plus),
    Token::Integer(String::from("1")),
    Token::RightBrace,
    Token::Eof,
  ];

  compare(input, expected)
}

#[test]
fn for_loops() {
  let input = String::from("for a in [1,2,3] { let x = a + 1 }");

  let expected = vec![
    Token::For,
    Token::Id(String::from("a")),
    Token::In,
    Token::LeftBracket,
    Token::Integer(String::from("1")),
    Token::Comma,
    Token::Integer(String::from("2")),
    Token::Comma,
    Token::Integer(String::from("3")),
    Token::RightBracket,
    Token::LeftBrace,
    Token::Let,
    Token::Id(String::from("x")),
    Token::Operator(Operator::Assign),
    Token::Id(String::from("a")),
    Token::Operator(Operator::Plus),
    Token::Integer(String::from("1")),
    Token::RightBrace,
    Token::Eof,
  ];

  compare(input, expected)
}

#[test]
fn array_indexes() {
  let input = String::from("[1,2,3,4][0]");

  let expected = vec![
    Token::LeftBracket,
    Token::Integer(String::from("1")),
    Token::Comma,
    Token::Integer(String::from("2")),
    Token::Comma,
    Token::Integer(String::from("3")),
    Token::Comma,
    Token::Integer(String::from("4")),
    Token::RightBracket,
    Token::LeftBracket,
    Token::Integer(String::from("0")),
    Token::RightBracket,
    Token::Eof,
  ];

  compare(input, expected)
}

#[test]
fn array() {
  let input = String::from("[1, 2, 3, 'leonardo', 'gurgel']");

  let expected = vec![
    Token::LeftBracket,
    Token::Integer(String::from("1")),
    Token::Comma,
    Token::Integer(String::from("2")),
    Token::Comma,
    Token::Integer(String::from("3")),
    Token::Comma,
    Token::String(String::from("leonardo")),
    Token::Comma,
    Token::String(String::from("gurgel")),
    Token::RightBracket,
    Token::Eof,
  ];

  compare(input, expected)
}

#[test]
fn string() {
  let input = String::from("\"leonardo gurgel\"");

  let expected = vec![Token::String(String::from("leonardo gurgel"))];

  compare(input, expected)
}

#[test]
fn multichar_operators() {
  let input = String::from(
    "10 == 10;
      10 != 9;
      ", // 10 >= 1;
         // 10 <= 50;
  );

  let expected = vec![
    Token::Integer(String::from("10")),
    Token::Operator(Operator::Equals),
    Token::Integer(String::from("10")),
    Token::Semicolon,
    Token::Integer(String::from("10")),
    Token::Operator(Operator::NotEquals),
    Token::Integer(String::from("9")),
    Token::Semicolon,
    Token::Eof,
  ];

  compare(input, expected)
}

#[test]
fn keywords() {
  let input = String::from(
    "if (5 < 10) {
        return true;
      } else {
        return false;
      }",
  );

  let expected = vec![
    Token::If,
    Token::LeftParen,
    Token::Integer(String::from("5")),
    Token::Operator(Operator::LessThan),
    Token::Integer(String::from("10")),
    Token::RightParen,
    Token::LeftBrace,
    Token::Return,
    Token::True,
    Token::Semicolon,
    Token::RightBrace,
    Token::Else,
    Token::LeftBrace,
    Token::Return,
    Token::False,
    Token::Semicolon,
    Token::RightBrace,
    Token::Eof,
  ];

  compare(input, expected)
}

#[test]
fn operators() {
  let input = String::from(
    "!-/*5;
      5 < 10 > 5;",
  );

  let expected_tokens = vec![
    Token::Operator(Operator::Bang),
    Token::Operator(Operator::Minus),
    Token::Operator(Operator::Slash),
    Token::Operator(Operator::Asterisk),
    Token::Integer(String::from("5")),
    Token::Semicolon,
    Token::Integer(String::from("5")),
    Token::Operator(Operator::LessThan),
    Token::Integer(String::from("10")),
    Token::Operator(Operator::GreaterThan),
    Token::Integer(String::from("5")),
    Token::Semicolon,
    Token::Eof,
  ];

  compare(input, expected_tokens)
}

#[test]
fn code() {
  let input = String::from(
    "
      let five = 5;
      let ten = 10;

      let add = fn(x, y) {
        x + y;
      };

      let result = add(five, ten);
      ",
  );

  let expected_tokens = vec![
    Token::Let,
    Token::Id(String::from("five")),
    Token::Operator(Operator::Assign),
    Token::Integer(String::from("5")),
    Token::Semicolon,
    Token::Let,
    Token::Id(String::from("ten")),
    Token::Operator(Operator::Assign),
    Token::Integer(String::from("10")),
    Token::Semicolon,
    Token::Let,
    Token::Id(String::from("add")),
    Token::Operator(Operator::Assign),
    Token::Function,
    Token::LeftParen,
    Token::Id(String::from("x")),
    Token::Comma,
    Token::Id(String::from("y")),
    Token::RightParen,
    Token::LeftBrace,
    Token::Id(String::from("x")),
    Token::Operator(Operator::Plus),
    Token::Id(String::from("y")),
    Token::Semicolon,
    Token::RightBrace,
    Token::Semicolon,
    Token::Let,
    Token::Id(String::from("result")),
    Token::Operator(Operator::Assign),
    Token::Id(String::from("add")),
    Token::LeftParen,
    Token::Id(String::from("five")),
    Token::Comma,
    Token::Id(String::from("ten")),
    Token::RightParen,
    Token::Semicolon,
    Token::Eof,
  ];

  compare(input, expected_tokens)
}

#[test]
fn symbols() {
  let input = String::from("=+(){},;");

  let expected_tokens = vec![
    Token::Operator(Operator::Assign),
    Token::Operator(Operator::Plus),
    Token::LeftParen,
    Token::RightParen,
    Token::LeftBrace,
    Token::RightBrace,
    Token::Comma,
    Token::Semicolon,
    Token::Eof,
  ];

  compare(input, expected_tokens)
}

fn compare(input: String, expected_tokens: Vec<Token>) {
  let lexer = Lexer::new(input);

  for expected in expected_tokens {
    let actual = lexer.next_token();
    assert_eq!(expected, actual);
  }
}
