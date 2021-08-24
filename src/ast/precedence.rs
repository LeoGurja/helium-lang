#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Precedence {
  Lowest,
  Assign,      // += | -= | *= | /= | =
  Equals,      // == | !=
  LessGreater, // >  | < | <= | >=
  Sum,         // +  | -
  Product,     // *
  Prefix,      // -X or !X
  Call,        // myFunction(X)
  Index,       // array[index]
}

impl Precedence {
  pub fn from(op: &str) -> Self {
    match op {
      "==" | "!=" => Self::Equals,
      "<" | ">" | "<=" | ">=" => Self::LessGreater,
      "+" | "-" => Self::Sum,
      "*" | "/" => Self::Product,
      _ => unreachable!(),
    }
  }
}
