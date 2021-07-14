use crate::ast::{Expression, Statement};
use crate::env::Env;
use crate::error::Error;
use crate::object::Object;
use crate::token::Operator;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Result = std::result::Result<Object, Error>;

pub struct Visitor {
  pub env: Rc<RefCell<Env>>,
}

impl Visitor {
  pub fn new() -> Self {
    Visitor { env: Env::global() }
  }

  pub fn from(env: Rc<RefCell<Env>>) -> Self {
    Visitor { env }
  }

  pub fn visit(&self, block: &Vec<Statement>) -> Result {
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
      Statement::Block(block) => self.visit_block(block),
      Statement::Null => unreachable!(),
      Statement::While(condition, block) => self.visit_while(condition, block),
      Statement::For(variable, iterable, block) => self.visit_for(variable, iterable, block),
      Statement::Expression(expression) => self.visit_expression(expression),
      Statement::Let(name, expression) => self.visit_variable_declaration(name, expression),
      Statement::Return(expression) => self.visit_return(expression),
    }
  }

  fn visit_block(&self, block: &Vec<Statement>) -> Result {
    let sub_visitor = Visitor::from(Env::local(self.env.clone()));
    sub_visitor.visit(block)
  }

  fn visit_for(&self, variable: &String, iterable: &Expression, block: &Statement) -> Result {
    let array = self.visit_expression(iterable)?;
    let mut evaluated = Object::Null;
    if let Object::Array(arr) = array {
      for i in &*arr.borrow() {
        self.env.borrow_mut().set(variable, i.clone());
        evaluated = self.visit_statement(block)?;
      }
      Ok(evaluated)
    } else {
      Err(Error::TypeError(String::from("array"), array))
    }
  }

  fn visit_while(&self, condition: &Expression, block: &Statement) -> Result {
    let mut response = Object::Null;
    while self.visit_expression(condition)?.is_truthy() {
      response = self.visit_statement(block)?;
    }
    Ok(response)
  }

  fn visit_variable_declaration(&self, name: &String, expression: &Expression) -> Result {
    let value = self.visit_expression(expression)?;
    self.env.borrow_mut().set(&name, value);
    Ok(Object::Null)
  }

  fn visit_return(&self, expression: &Option<Expression>) -> Result {
    Ok(Object::Return(Box::new(match expression {
      Some(value) => self.visit_expression(value)?,
      None => Object::Null,
    })))
  }

  fn visit_expression(&self, expression: &Expression) -> Result {
    match &expression {
      Expression::Hash(hash) => self.visit_hash(hash),
      Expression::Index(indexed, indexer) => self.visit_index(indexed, indexer),
      Expression::Array(expressions) => Ok(Object::Array(Rc::new(RefCell::new(
        self.visit_expressions(expressions)?,
      )))),
      Expression::Boolean(value) => Ok(Object::from(*value)),
      Expression::Integer(value) => Ok(Object::Integer(*value)),
      Expression::Call(function, args) => self.visit_call(function, args),
      Expression::Function(name, args, block) => self.visit_function_declaration(name, args, block),
      Expression::Id(name) => self.visit_variable(&name),
      Expression::If(condition, consequence, alternative) => {
        self.visit_if(condition, consequence, alternative)
      }
      Expression::Infix(infix, left, right) => self.visit_infix(infix, left, right),
      Expression::Prefix(prefix, expression) => self.visit_prefix(prefix, expression),
      Expression::String(value) => Ok(Object::String(value.clone())),
    }
  }

  fn visit_index(&self, left: &Expression, right: &Expression) -> Result {
    Ok(
      match (self.visit_expression(left)?, self.visit_expression(right)?) {
        (Object::Array(arr), Object::Integer(idx)) => arr
          .borrow()
          .get(idx as usize)
          .unwrap_or(&Object::Null)
          .clone(),
        (Object::Hash(hash), Object::String(string)) => {
          hash.borrow().get(&string).unwrap_or(&Object::Null).clone()
        }
        (left, right) => return Err(Error::IndexError(left, right)),
      },
    )
  }

  fn visit_hash(&self, key_values: &Vec<(Expression, Expression)>) -> Result {
    let mut hash = HashMap::new();
    for (key_expression, value_expression) in key_values {
      let key = match self.visit_expression(&key_expression)? {
        Object::Boolean(b) => b.to_string(),
        Object::Integer(i) => i.to_string(),
        Object::String(s) => s,
        obj => return Err(Error::UnsupportedHashKey(obj)),
      };
      hash.insert(key, self.visit_expression(&value_expression)?);
    }

    Ok(Object::Hash(Rc::new(RefCell::new(hash))))
  }

  fn visit_call(&self, function: &Expression, arg_values: &Vec<Expression>) -> Result {
    let function = self.visit_expression(&function)?;
    let args = self.visit_expressions(arg_values)?;

    match function {
      Object::Function(arg_names, block, env) => {
        self.visit_function_call(&arg_names, &args, &block, env)
      }
      Object::BuiltIn(function) => function(args),
      _ => Err(Error::CallError(function)),
    }
  }

  fn visit_expressions(
    &self,
    expressions: &[Expression],
  ) -> std::result::Result<Vec<Object>, Error> {
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
    block: &Statement,
    env: Rc<RefCell<Env>>,
  ) -> Result {
    if arg_names.len() != arg_values.len() {
      return Err(Error::WrongParameters(arg_names.len(), arg_values.len()));
    }
    let child_env = Env::local(env);
    for i in 0..arg_names.len() {
      child_env
        .borrow_mut()
        .set(&arg_names[i], arg_values[i].clone())
    }
    let sub_visitor = Visitor::from(child_env);
    sub_visitor.visit_statement(block)
  }

  fn visit_function_declaration(
    &self,
    name: &Option<String>,
    args: &Vec<String>,
    block: &Statement,
  ) -> Result {
    let function = Object::Function(args.clone(), block.clone(), self.env.clone());
    match name {
      Some(value) => self.env.borrow_mut().set(&value, function.clone()),
      None => (),
    };

    Ok(function)
  }

  fn visit_variable(&self, name: &str) -> Result {
    match self.env.borrow().get(name) {
      Some(value) => Ok(value),
      None => Err(Error::UndefinedVariable(name.to_string())),
    }
  }

  fn visit_if(
    &self,
    condition: &Expression,
    consequence: &Statement,
    alternative: &Option<Box<Statement>>,
  ) -> Result {
    if self.visit_expression(condition)?.is_truthy() {
      self.visit_statement(consequence)
    } else {
      match alternative {
        Some(statement) => self.visit_statement(statement),
        None => Ok(Object::Null),
      }
    }
  }

  fn visit_infix(
    &self,
    infix: &Operator,
    left_expression: &Expression,
    right_expression: &Expression,
  ) -> Result {
    let left = self.visit_expression(left_expression)?;
    let right = self.visit_expression(right_expression)?;
    Ok(match infix {
      Operator::Assign => match left_expression {
        Expression::Id(id) => {
          self.env.borrow_mut().update(id, right.clone());
          right
        }
        Expression::Index(indexed, index) => self.visit_index_assign(indexed, index, right)?,
        _ => return Err(Error::CannotAssign(left)),
      },
      Operator::Plus => left + right,
      Operator::Asterisk => left * right,
      Operator::Equals => Object::from(left == right),
      Operator::NotEquals => Object::from(left != right),
      Operator::GreaterThan => Object::from(left > right),
      Operator::LessThan => Object::from(left < right),
      Operator::Minus => left - right,
      Operator::Slash => left / right,
      _ => return Err(Error::UnknownOperator(infix.clone(), left)),
    })
  }

  fn visit_index_assign(
    &self,
    indexed: &Box<Expression>,
    index: &Box<Expression>,
    value: Object,
  ) -> Result {
    match (
      self.visit_expression(indexed)?,
      self.visit_expression(index)?,
    ) {
      (Object::Array(arr), Object::Integer(int)) => {
        arr.borrow_mut()[int as usize] = value.clone();
        Ok(value)
      }
      (Object::Hash(hash), Object::String(string)) => {
        hash.borrow_mut().insert(string, value.clone());
        Ok(value)
      }
      (left, right) => Err(Error::IndexError(left, right)),
    }
  }

  fn visit_prefix(&self, prefix: &Operator, expression: &Expression) -> Result {
    let obj = self.visit_expression(expression)?;
    Ok(match prefix {
      Operator::Bang => !obj,
      Operator::Minus => -obj,
      _ => return Err(Error::UnknownOperator(prefix.clone(), obj.clone())),
    })
  }
}
