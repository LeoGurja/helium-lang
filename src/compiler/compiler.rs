use crate::{
  ast::{Expression, Statement},
  bytecode::{ByteCode, Opcode},
  error::Error,
  object::Object,
};

pub struct Compiler {
  pub bytecode: ByteCode,
}

impl Compiler {
  pub fn new() -> Self {
    Compiler {
      bytecode: ByteCode {
        instructions: vec![],
        constants: vec![],
      },
    }
  }

  pub fn compile(&mut self, program: Vec<Statement>) -> Result<(), Error> {
    for statement in program {
      self.compile_statement(statement)?
    }

    Ok(())
  }

  fn compile_statement(&mut self, statement: Statement) -> Result<(), Error> {
    match statement {
      Statement::Expression(expression) => self.compile_expression(expression),
      _ => Ok(()),
    }
  }

  fn compile_expression(&mut self, expression: Expression) -> Result<(), Error> {
    match expression {
      Expression::Infix(operator, left, right) => self.compile_infix(operator, *left, *right),
      Expression::Prefix(operator, exp) => self.compile_prefix(operator, *exp),
      Expression::Integer(integer) => {
        let operand = vec![self.add_constant(Object::Integer(integer)) as u8];
        self.emit(Opcode::Constant, operand);
        Ok(())
      }
      _ => Ok(()),
    }
  }

  fn compile_infix(
    &mut self,
    operator: Operator,
    left: Expression,
    right: Expression,
  ) -> Result<(), Error> {
    self.compile_expression(left)?;
    self.compile_expression(right)?;
    Ok(())
  }

  fn compile_prefix(&mut self, operator: Operator, expression: Expression) -> Result<(), Error> {
    self.compile_expression(expression)
  }

  fn emit(&mut self, opcode: Opcode, operands: Vec<u8>) -> usize {
    let pos = self.bytecode.instructions.len();
    self.bytecode.instructions.push(opcode as u8);
    self.bytecode.instructions.extend(operands);
    pos
  }

  fn add_constant(&mut self, obj: Object) -> usize {
    self.bytecode.constants.push(obj);
    self.bytecode.constants.len() - 1
  }
}
