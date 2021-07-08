use super::*;

#[test]
fn next_token_on_array() {
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
  ];

  compare(input, expected)
}

#[test]
fn next_token_on_string() {
  let input = String::from("\"leonardo gurgel\"");

  let expected = vec![Token::String(String::from("leonardo gurgel"))];

  compare(input, expected)
}

#[test]
fn next_token_on_multichar_operators() {
  let input = String::from(
    "10 == 10;
      10 != 9;
      ", // 10 >= 1;
         // 10 <= 50;
  );

  let expected = vec![
    Token::Integer(String::from("10")),
    Token::Equals,
    Token::Integer(String::from("10")),
    Token::Semicolon,
    Token::Integer(String::from("10")),
    Token::NotEquals,
    Token::Integer(String::from("9")),
    Token::Semicolon,
    Token::Eof,
  ];

  compare(input, expected)
}

#[test]
fn next_token_on_keywords() {
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
    Token::LessThan,
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
fn next_token_on_operators() {
  let input = String::from(
    "!-/*5;
      5 < 10 > 5;",
  );

  let expected_tokens = vec![
    Token::Bang,
    Token::Minus,
    Token::Slash,
    Token::Asterisk,
    Token::Integer(String::from("5")),
    Token::Semicolon,
    Token::Integer(String::from("5")),
    Token::LessThan,
    Token::Integer(String::from("10")),
    Token::GreaterThan,
    Token::Integer(String::from("5")),
    Token::Semicolon,
    Token::Eof,
  ];

  compare(input, expected_tokens)
}

#[test]
fn next_token_on_code() {
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
    Token::Assign,
    Token::Integer(String::from("5")),
    Token::Semicolon,
    Token::Let,
    Token::Id(String::from("ten")),
    Token::Assign,
    Token::Integer(String::from("10")),
    Token::Semicolon,
    Token::Let,
    Token::Id(String::from("add")),
    Token::Assign,
    Token::Function,
    Token::LeftParen,
    Token::Id(String::from("x")),
    Token::Comma,
    Token::Id(String::from("y")),
    Token::RightParen,
    Token::LeftBrace,
    Token::Id(String::from("x")),
    Token::Plus,
    Token::Id(String::from("y")),
    Token::Semicolon,
    Token::RightBrace,
    Token::Semicolon,
    Token::Let,
    Token::Id(String::from("result")),
    Token::Assign,
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
fn lexer_next_token_on_symbols() {
  let input = String::from("=+(){},;");

  let expected_tokens = vec![
    Token::Assign,
    Token::Plus,
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
  let mut lexer = Lexer::new(input);

  for expected in expected_tokens {
    let actual = lexer.next_token();
    assert_eq!(expected, actual);
  }
}
