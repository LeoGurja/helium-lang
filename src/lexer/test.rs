use super::*;

#[test]
fn no_semicolons() {
  let input = "
  let x = 0

  fn main() { 
    for i in [1, 2] {
      x = x + i
    }

    if x > 6 {
      print(x)
    } else {
      print('too small')
    }

    0
  }";

  let expected = vec![
    Token::Let,
    Token::Id("x"),
    Token::Operator("="),
    Token::Integer(0),
    Token::Function,
    Token::Id("main"),
    Token::LeftParen,
    Token::RightParen,
    Token::LeftBrace,
    Token::For,
    Token::Id("i"),
    Token::In,
    Token::LeftBracket,
    Token::Integer(1),
    Token::Comma,
    Token::Integer(2),
    Token::RightBracket,
    Token::LeftBrace,
    Token::Id("x"),
    Token::Operator("="),
    Token::Id("x"),
    Token::Operator("+"),
    Token::Id("i"),
    Token::RightBrace,
    Token::If,
    Token::Id("x"),
    Token::Operator(">"),
    Token::Integer(6),
    Token::LeftBrace,
    Token::Id("print"),
    Token::LeftParen,
    Token::Id("x"),
    Token::RightParen,
    Token::RightBrace,
    Token::Else,
    Token::LeftBrace,
    Token::Id("print"),
    Token::LeftParen,
    Token::String("too small"),
    Token::RightParen,
    Token::RightBrace,
    Token::Integer(0),
    Token::RightBrace,
  ];

  compare(input, expected)
}

#[test]
fn hash_indexes() {
  let input = "hash['leonardo']";

  let expected = vec![
    Token::Id("hash"),
    Token::LeftBracket,
    Token::String("leonardo"),
    Token::RightBracket,
  ];

  compare(input, expected)
}

#[test]
fn hashes() {
  let input = "let x = {'name': 'leonardo', 'last_name': 'ferreira', 1: 1}";

  let expected = vec![
    Token::Let,
    Token::Id("x"),
    Token::Operator("="),
    Token::LeftBrace,
    Token::String("name"),
    Token::Colon,
    Token::String("leonardo"),
    Token::Comma,
    Token::String("last_name"),
    Token::Colon,
    Token::String("ferreira"),
    Token::Comma,
    Token::Integer(1),
    Token::Colon,
    Token::Integer(1),
    Token::RightBrace,
  ];

  compare(input, expected)
}

#[test]
fn while_loops() {
  let input = "let x = 0; while x < 10 { x = x + 1 }";

  let expected = vec![
    Token::Let,
    Token::Id("x"),
    Token::Operator("="),
    Token::Integer(0),
    Token::Semicolon,
    Token::While,
    Token::Id("x"),
    Token::Operator("<"),
    Token::Integer(10),
    Token::LeftBrace,
    Token::Id("x"),
    Token::Operator("="),
    Token::Id("x"),
    Token::Operator("+"),
    Token::Integer(1),
    Token::RightBrace,
  ];

  compare(input, expected)
}

#[test]
fn for_loops() {
  let input = "for a in [1,2,3] { let x = a + 1 }";

  let expected = vec![
    Token::For,
    Token::Id("a"),
    Token::In,
    Token::LeftBracket,
    Token::Integer(1),
    Token::Comma,
    Token::Integer(2),
    Token::Comma,
    Token::Integer(3),
    Token::RightBracket,
    Token::LeftBrace,
    Token::Let,
    Token::Id("x"),
    Token::Operator("="),
    Token::Id("a"),
    Token::Operator("+"),
    Token::Integer(1),
    Token::RightBrace,
  ];

  compare(input, expected)
}

#[test]
fn array_indexes() {
  let input = "[1,2,3,4][0]";

  let expected = vec![
    Token::LeftBracket,
    Token::Integer(1),
    Token::Comma,
    Token::Integer(2),
    Token::Comma,
    Token::Integer(3),
    Token::Comma,
    Token::Integer(4),
    Token::RightBracket,
    Token::LeftBracket,
    Token::Integer(0),
    Token::RightBracket,
  ];

  compare(input, expected)
}

#[test]
fn array() {
  let input = "[1, 2, 3, 'leonardo', 'gurgel']";

  let expected = vec![
    Token::LeftBracket,
    Token::Integer(1),
    Token::Comma,
    Token::Integer(2),
    Token::Comma,
    Token::Integer(3),
    Token::Comma,
    Token::String("leonardo"),
    Token::Comma,
    Token::String("gurgel"),
    Token::RightBracket,
  ];

  compare(input, expected)
}

#[test]
fn string() {
  let input = "\"leonardo gurgel\"";

  let expected = vec![Token::String("leonardo gurgel")];

  compare(input, expected)
}

#[test]
fn multichar_operators() {
  let input = "10 == 10;
      10 != 9;
      "; // 10 >= 1;
         // 10 <= 50;

  let expected = vec![
    Token::Integer(10),
    Token::Operator("=="),
    Token::Integer(10),
    Token::Semicolon,
    Token::Integer(10),
    Token::Operator("!="),
    Token::Integer(9),
    Token::Semicolon,
  ];

  compare(input, expected)
}

#[test]
fn keywords() {
  let input = "if (5 < 10) {
        return true;
      } else {
        return false;
      }";

  let expected = vec![
    Token::If,
    Token::LeftParen,
    Token::Integer(5),
    Token::Operator("<"),
    Token::Integer(10),
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
  ];

  compare(input, expected)
}

#[test]
fn operators() {
  let input = "!-/*5;
      5 < 10 > 5;";

  let expected_tokens = vec![
    Token::Operator("!"),
    Token::Operator("-"),
    Token::Operator("/"),
    Token::Operator("*"),
    Token::Integer(5),
    Token::Semicolon,
    Token::Integer(5),
    Token::Operator("<"),
    Token::Integer(10),
    Token::Operator(">"),
    Token::Integer(5),
    Token::Semicolon,
  ];

  compare(input, expected_tokens)
}

#[test]
fn code() {
  let input = "
      let five = 5;
      let ten = 10;

      let add = fn(x, y) {
        x + y;
      };

      let result = add(five, ten);
      ";

  let expected_tokens = vec![
    Token::Let,
    Token::Id("five"),
    Token::Operator("="),
    Token::Integer(5),
    Token::Semicolon,
    Token::Let,
    Token::Id("ten"),
    Token::Operator("="),
    Token::Integer(10),
    Token::Semicolon,
    Token::Let,
    Token::Id("add"),
    Token::Operator("="),
    Token::Function,
    Token::LeftParen,
    Token::Id("x"),
    Token::Comma,
    Token::Id("y"),
    Token::RightParen,
    Token::LeftBrace,
    Token::Id("x"),
    Token::Operator("+"),
    Token::Id("y"),
    Token::Semicolon,
    Token::RightBrace,
    Token::Semicolon,
    Token::Let,
    Token::Id("result"),
    Token::Operator("="),
    Token::Id("add"),
    Token::LeftParen,
    Token::Id("five"),
    Token::Comma,
    Token::Id("ten"),
    Token::RightParen,
    Token::Semicolon,
  ];

  compare(input, expected_tokens)
}

#[test]
fn symbols() {
  let input = "=+(){},;";

  let expected_tokens = vec![
    Token::Operator("="),
    Token::Operator("+"),
    Token::LeftParen,
    Token::RightParen,
    Token::LeftBrace,
    Token::RightBrace,
    Token::Comma,
    Token::Semicolon,
  ];

  compare(input, expected_tokens)
}

fn compare(input: &str, expected_tokens: Vec<Token>) {
  let mut lexer = lex(input);

  for expected in expected_tokens {
    let actual = lexer.next();
    assert_eq!(Some(expected), actual);
  }
  assert_eq!(None, lexer.next())
}
