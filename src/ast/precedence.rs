#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Precedence {
  Lowest,
  Assign,      // =
  Equals,      // == | !=
  LessGreater, // >  | < | <= | >=
  Sum,         // +  | -
  Product,     // *
  Prefix,      // -X or !X
  Call,        // myFunction(X)
  Index,       // array[index]
}

impl Precedence {
  pub fn from(operator: &str) -> Self {
    match operator {
      "=" | "+=" | "-=" | "*=" | "/=" => Precedence::Assign,
      "==" | "!=" => Precedence::Equals,
      ">" | "<" | ">=" | "<=" => Precedence::LessGreater,
      "+" | "-" => Precedence::Sum,
      "*" | "/" => Precedence::Product,
      _ => unreachable!(),
    }
  }
}
