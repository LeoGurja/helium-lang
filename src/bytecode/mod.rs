use crate::object::Object;
use std::fmt;

#[cfg(test)]
mod test;

pub struct ByteCode {
  pub instructions: Vec<u8>,
  pub constants: Vec<Object>,
}

#[repr(u8)]
pub enum Opcode {
  Constant,
}

impl fmt::Display for Opcode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Constant => "Constant",
      }
    )
  }
}

impl Opcode {
  pub fn size(&self) -> u8 {
    match self {
      Self::Constant => 5,
    }
  }

  pub fn make(opcode: Self, operand: i64) -> Vec<u8> {
    let mut vec = i64::to_be_bytes(operand).to_vec();
    vec.insert(0, opcode as u8);
    vec
  }
}
