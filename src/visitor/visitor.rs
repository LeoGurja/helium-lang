use crate::ast::{Expression, Statement};
use crate::env::Env;
use crate::errors::EvalError;
use crate::object::{Object, Type};
use crate::token::Operator;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type Result = std::result::Result<Object, EvalError>;

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
    let mut result = Object::NULL;
    for statement in block {
      result = self.visit_statement(statement)?;
      if let Type::Return(value) = result.content {
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
    let mut evaluated = Object::NULL;
    if let Type::Array(arr) = array.content {
      for i in arr {
        self.env.borrow_mut().set(variable, i);
        evaluated = self.visit_statement(block)?;
      }
      Ok(evaluated)
    } else {
      Err(EvalError::TypeError(String::from("array"), array))
    }
  }

  fn visit_while(&self, condition: &Expression, block: &Statement) -> Result {
    let mut response = Object::NULL;
    while self.visit_expression(condition)?.is_truthy() {
      response = self.visit_statement(block)?;
    }
    Ok(response)
  }

  fn visit_variable_declaration(&self, name: &String, expression: &Expression) -> Result {
    let value = self.visit_expression(expression)?;
    self.env.borrow_mut().set(&name, value);
    Ok(Object::NULL)
  }

  fn visit_return(&self, expression: &Option<Expression>) -> Result {
    Ok(Object::new(Type::Return(Box::new(match expression {
      Some(value) => self.visit_expression(value)?,
      None => Object::NULL,
    }))))
  }

  fn visit_expression(&self, expression: &Expression) -> Result {
    Ok(match &expression {
      Expression::Hash(hash) => self.visit_hash(hash)?,
      Expression::Index(indexed, indexer) => self.visit_index(
        self.visit_expression(indexed)?,
        self.visit_expression(indexer)?,
      )?,
      Expression::Array(expressions) => {
        Object::new(Type::Array(self.visit_expressions(expressions)?))
      }
      Expression::Boolean(value) => {
        if *value {
          Object::TRUE
        } else {
          Object::FALSE
        }
      }
      Expression::Integer(value) => Object::new(Type::Integer(*value)),
      Expression::Call(function, args) => self.visit_call(function, args)?,
      Expression::Function(name, args, block) => {
        self.visit_function_declaration(name, args, block)?
      }
      Expression::Id(name) => self.visit_variable(&name)?,
      Expression::If(condition, consequence, alternative) => {
        self.visit_if(condition, consequence, alternative)?
      }
      Expression::Infix(infix, left, right) => self.visit_infix(infix, left, right)?,
      Expression::Prefix(prefix, expression) => self.visit_prefix(prefix, expression)?,
      Expression::String(value) => Object::new(Type::String(value.clone())),
    })
  }

  fn visit_hash(&self, key_values: &Vec<(Expression, Expression)>) -> Result {
    let mut hash = HashMap::new();
    for (key_expression, value_expression) in key_values {
      let key = match self.visit_expression(&key_expression)?.content {
        Type::Boolean(b) => b.to_string(),
        Type::Integer(i) => i.to_string(),
        Type::String(s) => s,
        obj => return Err(EvalError::UnsupportedHashKey(obj)),
      };
      hash.insert(key, self.visit_expression(&value_expression)?);
    }

    Ok(Object::new(Type::Hash(hash)))
  }

  fn visit_call(&self, function: &Expression, arg_values: &Vec<Expression>) -> Result {
    let function = self.visit_expression(&function)?;
    let args = self.visit_expressions(arg_values)?;

    match function.content {
      Type::Function(arg_names, block, env) => {
        self.visit_function_call(&arg_names, &args, &block, env)
      }
      Type::BuiltIn(function) => function(args),
      _ => Err(EvalError::CallError(function)),
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
    block: &Statement,
    env: Rc<RefCell<Env>>,
  ) -> Result {
    if arg_names.len() != arg_values.len() {
      return Err(EvalError::WrongParameters(
        arg_names.len(),
        arg_values.len(),
      ));
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
    let function = Object::new(Type::Function(
      args.clone(),
      block.clone(),
      self.env.clone(),
    ));
    match name {
      Some(value) => self.env.borrow_mut().set(&value, function.clone()),
      None => (),
    };

    Ok(function)
  }

  fn visit_variable(&self, name: &str) -> Result {
    match self.env.borrow().get(name) {
      Some(value) => Ok(value),
      None => Err(EvalError::UndefinedVariable(name.to_string())),
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
        None => Ok(Object::NULL),
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
    match infix {
      Operator::Assign => match left_expression {
        Expression::Id(id) => {
          let value = self.visit_expression(right_expression)?;
          self.env.borrow_mut().update(id, value.clone())?;
          Ok(value)
        }
        _ => Err(EvalError::CannotAssign(left)),
      },
      Operator::Plus => left.add(self.visit_expression(right_expression)?),
      Operator::Asterisk => left.multiply(self.visit_expression(right_expression)?),
      Operator::Equals => Ok(if left == self.visit_expression(right_expression)? {
        Object::TRUE
      } else {
        Object::FALSE
      }),
      Operator::NotEquals => Ok(if left != self.visit_expression(right_expression)? {
        Object::TRUE
      } else {
        Object::FALSE
      }),
      Operator::GreaterThan => left.greater_than(self.visit_expression(right_expression)?),
      Operator::LessThan => left.less_than(self.visit_expression(right_expression)?),
      Operator::Minus => left.subtract(self.visit_expression(right_expression)?),
      Operator::Slash => left.divide(self.visit_expression(right_expression)?),
      _ => Err(EvalError::UnknownOperator(Box::new(infix.clone()), left)),
    }
  }

  fn visit_index(&self, array_obj: Object, index_obj: Object) -> Result {
    match (&array_obj.content, &index_obj.content) {
      (Type::Hash(hash), index) => Ok(match hash.get(&index.to_string()) {
        Some(obj) => obj.clone(),
        None => Object::NULL,
      }),
      (Type::Array(array), Type::Integer(index)) => Ok(match array.get(*index as usize) {
        Some(obj) => obj.clone(),
        None => Object::NULL,
      }),
      _ => Err(EvalError::IndexError(array_obj, index_obj)),
    }
  }

  fn visit_prefix(&self, prefix: &Operator, expression: &Expression) -> Result {
    let obj = self.visit_expression(expression)?;
    match prefix {
      Operator::Bang => Ok(obj.not()),
      Operator::Minus => Ok(obj.negative()?),
      _ => Err(EvalError::UnknownOperator(
        Box::new(prefix.clone()),
        obj.clone(),
      )),
    }
  }
}
