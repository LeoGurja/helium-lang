use crate::{
  ast::{Expression, Statement},
  env,
  error::Error,
  object::Object,
};
use std::{collections::HashMap, rc::Rc};

type Result<T> = std::result::Result<T, Error>;

pub struct Visitor {
  pub env: env::Env,
}

impl Visitor {
  pub fn new() -> Self {
    Visitor { env: env::global() }
  }

  pub fn from(env: env::Env) -> Self {
    Visitor { env }
  }

  pub fn visit(&self, block: &[Statement]) -> Result<Object> {
    let mut result = Object::Null;
    for statement in block {
      result = self.visit_statement(statement)?;
      if let Object::Return(value) = result {
        return Ok(*value);
      }
    }

    Ok(result)
  }

  fn visit_statement(&self, statement: &Statement) -> Result<Object> {
    match statement {
      Statement::Block(block) => self.visit_block(block, env::local(self.env.clone())),
      Statement::Null => unreachable!(),
      Statement::WhileLoop(condition, block) => self.visit_while(condition, block),
      Statement::ForLoop(variable, iterable, block) => self.visit_for(variable, iterable, block),
      Statement::Expression(expression) => self.visit_expression(expression),
      Statement::VariableDeclaration(name, expression) => {
        self.visit_variable_declaration(&name, expression)
      }
      Statement::Return(expression) => self.visit_return(expression),
    }
  }

  fn visit_block(&self, block: &[Statement], env: env::Env) -> Result<Object> {
    let sub_visitor = Visitor::from(env);
    sub_visitor.visit(block)
  }

  fn visit_for(&self, variable: &str, iterable: &Expression, block: &Statement) -> Result<Object> {
    let array = self.visit_expression(iterable)?;
    let mut evaluated = Object::Null;
    if let Object::Array(arr) = array {
      for i in arr {
        self.env.set(variable, i);
        evaluated = self.visit_statement(block)?;
      }
      Ok(evaluated)
    } else {
      Err(Error::type_error("an array", array))
    }
  }

  fn visit_while(&self, condition: &Expression, block: &Statement) -> Result<Object> {
    let mut response = Object::Null;
    while self.visit_expression(condition)?.is_truthy() {
      response = self.visit_statement(block)?;
    }
    Ok(response)
  }

  fn visit_variable_declaration(&self, name: &str, expression: &Expression) -> Result<Object> {
    let value = self.visit_expression(expression)?;
    self.env.set(&name, value);
    Ok(Object::Null)
  }

  fn visit_return(&self, expression: &Expression) -> Result<Object> {
    Ok(Object::Return(Box::new(self.visit_expression(expression)?)))
  }

  fn visit_expression(&self, expression: &Expression) -> Result<Object> {
    match expression {
      Expression::Null => Ok(Object::Null),
      Expression::Hash(hash) => self.visit_hash(hash),
      Expression::Index(indexed, indexer) => self.visit_index(indexed, indexer),
      Expression::Array(expressions) => Ok(Object::Array(self.visit_expressions(expressions)?)),
      Expression::True => Ok(Object::True),
      Expression::False => Ok(Object::False),
      Expression::Integer(value) => Ok(Object::Integer(*value)),
      Expression::Call(function, args) => self.visit_call(&function, args),
      Expression::Function(name, args, block) => self.visit_function_declaration(name, args, block),
      Expression::Id(name) => self.visit_variable(&name),
      Expression::Conditional(condition, consequence, alternative) => {
        self.visit_conditional(&condition, &consequence, alternative)
      }
      Expression::Infix(infix, left, right) => self.visit_infix(&infix, &left, &right),
      Expression::Prefix(prefix, expression) => self.visit_prefix(&prefix, &expression),
      Expression::String(value) => Ok(Object::String(value.clone())),
    }
  }

  fn visit_index(&self, left: &Expression, right: &Expression) -> Result<Object> {
    Ok(
      match (self.visit_expression(left)?, self.visit_expression(right)?) {
        (Object::Array(arr), Object::Integer(idx)) => {
          arr.get(idx as usize).unwrap_or(&Object::Null).clone()
        }
        (Object::Hash(hash), Object::String(string)) => {
          hash.get(&string).unwrap_or(&Object::Null).clone()
        }
        (left, right) => return Err(Error::index_error(left, right)),
      },
    )
  }

  fn visit_hash(&self, key_values: &[(Expression, Expression)]) -> Result<Object> {
    let mut hash = HashMap::new();
    for (key_expression, value_expression) in key_values {
      let key = match self.visit_expression(&key_expression)? {
        Object::Integer(i) => i.to_string(),
        Object::String(s) => s,
        obj => return Err(Error::index_error(Object::Hash(hash), obj)),
      };
      hash.insert(key, self.visit_expression(&value_expression)?);
    }

    Ok(Object::Hash(hash))
  }

  fn visit_call(&self, function: &Expression, arg_values: &[Expression]) -> Result<Object> {
    self.execute_call(
      &self.visit_expression(function)?,
      self.visit_expressions(arg_values)?,
    )
  }

  fn execute_call(&self, obj: &Object, args: Vec<Object>) -> Result<Object> {
    match obj {
      Object::Function(arg_names, block, env) => {
        self.visit_function_call(arg_names, &args, block.as_ref(), env.clone())
      }
      Object::BuiltIn(function) => function(args),
      _ => Err(Error::call_error(obj.clone())),
    }
  }

  fn visit_expressions(
    &self,
    expressions: &[Expression],
  ) -> std::result::Result<Vec<Object>, Error> {
    let mut results = vec![];
    for expression in expressions {
      results.push(self.visit_expression(&expression)?);
    }
    Ok(results)
  }

  fn visit_function_call(
    &self,
    arg_names: &[String],
    arg_values: &[Object],
    block: &Statement,
    env: env::Env,
  ) -> Result<Object> {
    if arg_names.len() != arg_values.len() {
      return Err(Error::wrong_parameters(arg_names.len(), arg_values.len()));
    }
    let child_env = env::local(env);
    for i in 0..arg_names.len() {
      child_env.set(&arg_names[i], arg_values[i].clone())
    }
    let sub_visitor = Visitor::from(child_env);
    sub_visitor.visit_statement(block)
  }

  fn visit_function_declaration(
    &self,
    name: &Option<String>,
    args: &[String],
    block: &Rc<Statement>,
  ) -> Result<Object> {
    let function = Object::Function(args.to_owned(), block.clone(), self.env.clone());
    match name {
      Some(value) => self.env.set(&value, function.clone()),
      None => (),
    };

    Ok(function)
  }

  fn visit_variable(&self, name: &str) -> Result<Object> {
    let value = self.env.get(name);
    match value {
      Some(value) => Ok(value.clone()),
      None => Err(Error::undefined_variable(name)),
    }
  }

  fn visit_conditional(
    &self,
    condition: &Expression,
    consequence: &Statement,
    alternative: &Option<Box<Statement>>,
  ) -> Result<Object> {
    if self.visit_expression(condition)?.is_truthy() {
      self.visit_statement(consequence)
    } else {
      match alternative {
        Some(statement) => self.visit_statement(&statement),
        None => Ok(Object::Null),
      }
    }
  }

  fn visit_infix(
    &self,
    infix: &str,
    left_expression: &Expression,
    right_expression: &Expression,
  ) -> Result<Object> {
    let left = self.visit_expression(left_expression)?;
    let right = self.visit_expression(right_expression)?;

    match infix {
      "+" => left.add(right),
      "*" => left.multiply(right),
      "-" => left.subtract(right),
      "/" => left.divide(right),
      "==" => Ok(Object::boolean(left == right)),
      "!=" => Ok(Object::boolean(left != right)),
      ">" => Ok(Object::boolean(left > right)),
      "<" => Ok(Object::boolean(left < right)),
      "<=" => Ok(Object::boolean(left <= right)),
      ">=" => Ok(Object::boolean(left >= right)),
      _ => Err(Error::unknown_operator(infix.to_string(), left)),
    }
  }

  fn visit_prefix(&self, prefix: &str, expression: &Expression) -> Result<Object> {
    let obj = self.visit_expression(expression)?;
    Ok(match prefix {
      "!" => obj.not(),
      "-" => obj.negate()?,
      _ => return Err(Error::unknown_operator(prefix.to_string(), obj)),
    })
  }
}
