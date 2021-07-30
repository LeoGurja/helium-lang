use super::*;
use crate::{
  ast::{Expression, Statement},
  bytecode::{ByteCode, Opcode},
  error::Error,
  object::Object,
};

#[test]
fn integer_arithmetic() {
  let input = vec![Statement::from(Expression::infix(
    Operator::Plus,
    Expression::Integer(1),
    Expression::Integer(2),
  ))];
  let compiler = Compiler::new();
  let bytecode = compiler.compile(input);

  match compiler.compile(input) {
    Ok(_) => {
      assert_eq!(
        compiler.bytecode.instructions,
        vec![Opcode::Constant as u8, Opcode::Constant as u8]
      );

      assert_eq!(
        compiler.bytecode.constants,
        vec![Object::Integer(1), Object::Integer(2)]
      )
    }
    Err(err) => panic!(err.to_string()),
  }
}
