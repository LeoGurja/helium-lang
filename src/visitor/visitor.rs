use super::env::{Env, Link};
use super::eval_error::{EvalError, Result};
use crate::ast::{Block, Expression, Infix, Prefix, Statement};
use crate::object::Object;

pub struct Visitor {
  pub env: Env,
}

impl Visitor {
  pub fn new() -> Self {
    Visitor { env: Env::global() }
  }

  fn from(env: Env) -> Self {
    Visitor { env }
  }

  pub fn visit(&self, block: &Block) -> Result {
    let mut result = Object::Null;
    for statement in block {
      result = self.visit_statement(statement)?;
      if let Object::Return(value) = result {
        return Ok(*value);
      }
    }

    Ok(result)
  }

  fn visit_statement(&self, statement: &Statement) -> Result {
    match statement {
      Statement::Expression(expression) => self.visit_expression(expression),
      Statement::Let(name, expression) => self.visit_variable_declaration(name, expression),
      Statement::Return(expression) => self.visit_return(expression),
    }
  }

  fn visit_variable_declaration(&self, name: &String, expression: &Expression) -> Result {
    let value = self.visit_expression(expression)?;
    self.env.set(&name, value);
    Ok(Object::Null)
  }

  fn visit_return(&self, expression: &Option<Expression>) -> Result {
    Ok(Object::Return(Box::new(match expression {
      Some(value) => self.visit_expression(value)?,
      None => Object::Null,
    })))
  }

  fn visit_expression(&self, expression: &Expression) -> Result {
    Ok(match &expression {
      Expression::Boolean(value) => {
        if *value {
          Object::TRUE
        } else {
          Object::FALSE
        }
      }
      Expression::Integer(value) => Object::Integer(*value),
      Expression::Call(name, args) => self.visit_call(name, args)?,
      Expression::Function(name, args, block) => {
        self.visit_function_declaration(name, args, block)?
      }
      Expression::Id(name) => self.visit_variable(&name)?,
      Expression::If(condition, consequence, alternative) => {
        self.visit_if(condition, consequence, alternative)?
      }
      Expression::Infix(infix, left, right) => self.visit_infix(infix, left, right)?,
      Expression::Prefix(prefix, expression) => self.visit_prefix(prefix, expression)?,
      Expression::String(value) => Object::String(value.clone()),
    })
  }

  fn visit_call(&self, name: &String, arg_values: &Vec<Expression>) -> Result {
    let function = self.visit_variable(&name)?;
    let args = self.visit_expressions(arg_values)?;

    match function {
      Object::Function(arg_names, block, env) => {
        self.visit_function_call(&arg_names, &args, &block, env)
      }
      Object::BuiltIn(function) => function(args),
      _ => Err(EvalError::CallError(name.clone())),
    }
  }

  fn visit_expressions(
    &self,
    expressions: &[Expression],
  ) -> std::result::Result<Vec<Object>, EvalError> {
    let mut results = vec![];
    for expression in expressions {
      results.push(self.visit_expression(expression)?);
    }
    Ok(results)
  }

  fn visit_function_call(
    &self,
    arg_names: &Vec<String>,
    arg_values: &Vec<Object>,
    block: &Block,
    env: Env,
  ) -> Result {
    if arg_names.len() != arg_values.len() {
      return Err(EvalError::WrongParameters(
        arg_names.len(),
        arg_values.len(),
      ));
    }
    let child_env = Env::local(&env);
    for i in 0..arg_names.len() {
      child_env.set(&arg_names[i], arg_values[i].clone())
    }
    let sub_visitor = Visitor::from(child_env);
    sub_visitor.visit(block)
  }

  fn visit_function_declaration(
    &self,
    name: &Option<String>,
    args: &Vec<String>,
    block: &Block,
  ) -> Result {
    let function = Object::Function(args.clone(), block.clone(), self.env.clone());
    match name {
      Some(value) => self.env.set(&value, function.clone()),
      None => (),
    };

    Ok(function)
  }

  fn visit_variable(&self, name: &str) -> Result {
    match self.env.get(name) {
      Some(value) => Ok(value),
      None => Err(EvalError::UndefinedVariable(name.to_string())),
    }
  }

  fn visit_if(
    &self,
    condition: &Expression,
    consequence: &Block,
    alternative: &Option<Block>,
  ) -> Result {
    Ok(match self.visit_expression(condition)? {
      Object::Boolean(false) | Object::Null => match alternative {
        Some(block) => self.visit(block)?,
        None => Object::Null,
      },
      _ => self.visit(consequence)?,
    })
  }

  fn visit_infix(
    &self,
    infix: &Infix,
    left_expression: &Expression,
    right_expression: &Expression,
  ) -> Result {
    let left = self.visit_expression(left_expression)?;
    match infix {
      Infix::Plus => left.add(self.visit_expression(right_expression)?),
      Infix::Asterisk => left.multiply(self.visit_expression(right_expression)?),
      Infix::Equals => Ok(if left == self.visit_expression(right_expression)? {
        Object::TRUE
      } else {
        Object::FALSE
      }),
      Infix::NotEquals => Ok(if left != self.visit_expression(right_expression)? {
        Object::TRUE
      } else {
        Object::FALSE
      }),
      Infix::GreaterThan => left.greater_than(self.visit_expression(right_expression)?),
      Infix::LessThan => left.less_than(self.visit_expression(right_expression)?),
      Infix::Minus => left.subtract(self.visit_expression(right_expression)?),
      Infix::Slash => left.divide(self.visit_expression(right_expression)?),
    }
  }

  fn visit_prefix(&self, prefix: &Prefix, expression: &Expression) -> Result {
    match prefix {
      Prefix::Bang => Ok(self.visit_expression(expression)?.not()),
      Prefix::Minus => self.visit_expression(expression)?.negative(),
    }
  }
}
