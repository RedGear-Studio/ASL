use super::ast::{Statement, Expression, BinaryOperator, DataType, UnaryOperator, Literal};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Variable {
    data_type: DataType,
    value: Value,
}
#[derive(Debug)]
pub struct Scope (u32);
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}
impl Value {
    pub fn is_boolean(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            _ => false,
        }
    }
    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }
}
#[derive(Debug)]
pub enum EvalError {
    /// Expected, Found
    InvalidDataType,
    InvalidOperator,
    /// Found
    InvalidIdentifier(String),
    InvalidLiteral,
    InvalidExpression,
    InvalidStatement,
}
impl Display for EvalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            EvalError::InvalidDataType => write!(f, "Invalid data type"),
            EvalError::InvalidOperator => write!(f, "Invalid operator"),
            EvalError::InvalidIdentifier(id) => write!(f, "Error: \"Invalid identifier\":\n\t {} Not found", id),
            EvalError::InvalidLiteral => write!(f, "Invalid literal"),
            EvalError::InvalidExpression => write!(f, "Invalid expression"),
            EvalError::InvalidStatement => write!(f, "Invalid statement"),
        }
    }
}
pub enum EvalResult {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Sucess,
    Null,
}
impl Display for EvalResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            EvalResult::Number(number) => write!(f, "{}", number),
            EvalResult::String(string) => write!(f, "{}", string),
            EvalResult::Boolean(boolean) => write!(f, "{}", boolean),
            EvalResult::Identifier(identifier) => write!(f, "{}", identifier),
            EvalResult::Sucess => write!(f, "sucess"),
            EvalResult::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    pub variables: Vec<(String, Variable, Scope)>
}
impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            variables: vec![],
        }
    }
    fn get_variable_value(&self, identifier: String) -> Option<&Value> {
        let len = self.variables.len();
        for i in 0..len {
            println!("For Loop:\n\t\"{}\": \"{}\"", self.variables[len -1 -i].0, identifier);
            if self.variables[len -1 -i].0 == identifier {
                println!("If:\n\t\"{}\": \"{:?}\"", identifier, self.variables[len -1 -i].1.value);
                return Some(&self.variables[len -1 -i].1.value);
            }
        }
        return None;
    }
    pub fn new_boolean(&mut self, identifier: String, value: bool, scope: u32) -> Result<(), EvalError> {
        if self.variables.iter().any(|variable| variable.0 == identifier) {
            return Err(EvalError::InvalidIdentifier(identifier));
        }
        self.variables.push((identifier, Variable {
            data_type: DataType::Boolean,
            value: Value::Boolean(value),
        }, Scope(scope)));
        return Ok(());
    }
    fn new_number(&mut self, identifier: String, value: f64, scope: u32) -> Result<(), EvalError> {
        if self.variables.iter().any(|variable| variable.0 == identifier) {
            return Err(EvalError::InvalidIdentifier(identifier));
        }
        self.variables.push((identifier, Variable {
            data_type: DataType::Float,
            value: Value::Number(value),
        }, Scope(scope)));
        return Ok(());
    }
    fn new_string(&mut self, identifier: String, value: String, scope: u32) -> Result<(), EvalError> {
        if self.variables.iter().any(|variable| variable.0 == identifier) {
            return Err(EvalError::InvalidIdentifier(identifier));
        }
        self.variables.push((identifier, Variable {
            data_type: DataType::String,
            value: Value::String(value),
        }, Scope(scope)));
        return Ok(());
    }
    fn set_variable_value(&mut self, identifier: String, value: Value) -> Result<(), EvalError> {
        if let Some(variable) = self.variables
            .iter_mut()
            .find(|variable| variable.0 == identifier) {
            //Typechecking
            match variable.1.data_type {
                DataType::Boolean => {
                    if value.is_boolean() {
                        variable.1.value = value;
                        return Ok(());
                    }
                },
                DataType::Float => {
                    if value.is_number() {
                        variable.1.value = value;
                        return Ok(());
                    }
                },
                DataType::String => {
                    if value.is_string() {
                        variable.1.value = value;
                        return Ok(());
                    }
                },
                _ => ()
            }
            return Err(EvalError::InvalidDataType);
        }
        return Err(EvalError::InvalidIdentifier(identifier));
    }
    fn drop_scope(&mut self, scope: u32) -> Result<EvalResult, EvalError> {
        // Remove all the variable who have the scope
        self.variables.retain(|variable| variable.2.0 != scope);
        return Ok(EvalResult::Sucess);
    }
    pub fn eval(&mut self, program: Vec<Statement>, scope: u32) -> Result<EvalResult, EvalError> {
        for statement in program {
            match statement {
                Statement::PrintStatement(expression) => {
                    let result = self.eval_expression(expression)?;
                    match result {
                        EvalResult::Number(number) => println!("{}", number),
                        EvalResult::String(string) => println!("{}", string),
                        EvalResult::Boolean(boolean) => println!("{}", boolean),
                        EvalResult::Identifier(identifier) => {
                            let value = self.get_variable_value(identifier).unwrap();
                            match value {
                                Value::Number(number) => println!("{}", number),
                                Value::String(string) => println!("{}", string),
                                Value::Boolean(boolean) => println!("{}", boolean),
                                Value::Null => println!("null"),
                            }
                        },
                        EvalResult::Sucess => println!("sucess"),
                        EvalResult::Null => println!("null"),
                    }
                },
                Statement::IfStatement { cond_expr, body_expr, else_expr } => {
                    let result = self.eval_expression(cond_expr)?;
                    match result {
                        EvalResult::Boolean(boolean) => {
                            if boolean {
                                self.eval(body_expr, scope + 1)?;
                            } else if else_expr.is_some() {
                                self.eval(else_expr.unwrap(), scope + 1)?;
                            }
                        },
                        _ => return Err(EvalError::InvalidExpression),
                    }
                },
                Statement::VariableDeclaration { identifier, value, data_type } => {
                    let value = if value.is_some() {
                        self.eval_expression(value.unwrap())?
                    } else {
                        EvalResult::Null
                    };
                    match data_type {
                        DataType::Int => {
                            match value {
                                EvalResult::Number(number) => {
                                    self.new_number(identifier, number as i64 as f64, scope)?;
                                }
                                _ => return Err(EvalError::InvalidDataType),
                            }
                        },
                        DataType::Float => {
                            match value {
                                EvalResult::Number(number) => {
                                    self.new_number(identifier, number, scope)?;
                                }
                                _ => return Err(EvalError::InvalidDataType),
                            }
                        },
                        DataType::String => {
                            match value {
                                EvalResult::String(string) => {
                                    self.new_string(identifier, string, scope)?;
                                }
                                _ => return Err(EvalError::InvalidDataType),
                            }
                        },
                        DataType::Boolean => {
                            match value {
                                EvalResult::Boolean(boolean) => {
                                    self.new_boolean(identifier, boolean, scope)?;
                                }
                                _ => return Err(EvalError::InvalidDataType),
                            }
                        },
                        _ => return Err(EvalError::InvalidDataType),
                    }
                },
                Statement::WhileLoop { cond_expr, body_expr } => {
                    loop {
                        let result = self.eval_expression(cond_expr.clone())?;
                        match result {
                            EvalResult::Boolean(boolean) => {
                                if boolean {
                                    self.eval(body_expr.clone(), scope + 1)?;
                                } else {
                                    break;
                                }
                            },
                            _ => {
                                return Err(EvalError::InvalidExpression)
                            },
                        }
                    }
                },
                Statement::Assignment { identifier, value } => {
                    let value = self.eval_expression(value)?;
                    match value {
                        EvalResult::Number(number) => {
                            self.set_variable_value(identifier, Value::Number(number))?;
                        }
                        EvalResult::String(string) => {
                            self.set_variable_value(identifier, Value::String(string))?;
                        }
                        EvalResult::Boolean(boolean) => {
                            self.set_variable_value(identifier, Value::Boolean(boolean))?;
                        }
                        _ => return Err(EvalError::InvalidDataType),
                    }
                },
            }
        }
        self.drop_scope(scope)?;
        return Ok(EvalResult::Sucess);
    }

    fn eval_expression(&mut self, expression: Expression) -> Result<EvalResult, EvalError> {
        match expression {
            Expression::BinaryOp(left, op, right) => {
                let left = self.eval_expression(*left)?;
                let right = self.eval_expression(*right)?;
                match op {
                    // Arithmetic
                    BinaryOperator::Plus => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => {
                                return Ok(EvalResult::Number(left + right));
                            },
                            (EvalResult::String(left), EvalResult::String(right)) => {
                                return Ok(EvalResult::String(left + &right));
                            },
                            (EvalResult::Identifier(left), EvalResult::Identifier(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                let right = self.get_variable_value(right).unwrap().clone();
                                match (left, right) {
                                    (Value::Number(left), Value::Number(right)) => {
                                        return Ok(EvalResult::Number(left + right));
                                    },
                                    (Value::String(left), Value::String(right)) => {
                                        return Ok(EvalResult::String(left + &right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::Number(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                match left {
                                    Value::Number(left) => {
                                        return Ok(EvalResult::Number(left + right));
                                    },
                                    Value::String(left) => {
                                        return Ok(EvalResult::String(left + &right.to_string()));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::String(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                match left {
                                    Value::Number(left) => {
                                        return Ok(EvalResult::String(left.to_string() + &right));
                                    },
                                    Value::String(left) => {
                                        return Ok(EvalResult::String(left + &right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Number(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap();
                                match right {
                                    Value::Number(right) => {
                                        return Ok(EvalResult::Number(left + right));
                                    },
                                    Value::String(right) => {
                                        return Ok(EvalResult::String(left.to_string() + &right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::String(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap();
                                match right {
                                    Value::Number(right) => {
                                        return Ok(EvalResult::String(left + &right.to_string()));
                                    },
                                    Value::String(right) => {
                                        return Ok(EvalResult::String(left + &right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            _ => return Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::Minus => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => {
                                return Ok(EvalResult::Number(left - right));
                            },
                            (EvalResult::Identifier(left), EvalResult::Identifier(right)) => {
                                let left = self.get_variable_value(left).unwrap();
                                let right = self.get_variable_value(right).unwrap();
                                match (left, right) {
                                    (Value::Number(left), Value::Number(right)) => {
                                        return Ok(EvalResult::Number(left - right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::Number(right)) => {
                                let left = self.get_variable_value(left).unwrap();
                                match left {
                                    Value::Number(left) => {
                                        return Ok(EvalResult::Number(left - right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Number(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap();
                                match right {
                                    Value::Number(right) => {
                                        return Ok(EvalResult::Number(left - right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            _ => return Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::Slash => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => {
                                return Ok(EvalResult::Number(left / right));
                            },
                            (EvalResult::Identifier(left), EvalResult::Identifier(right)) => {
                                let left = self.get_variable_value(left).unwrap();
                                let right = self.get_variable_value(right).unwrap();
                                match (left, right) {
                                    (Value::Number(left), Value::Number(right)) => {
                                        return Ok(EvalResult::Number(left / right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::Number(right)) => {
                                let left = self.get_variable_value(left).unwrap();
                                match left {
                                    Value::Number(left) => {
                                        return Ok(EvalResult::Number(left / right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Number(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap();
                                match right {
                                    Value::Number(right) => {
                                        return Ok(EvalResult::Number(left / right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            _ => return Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::Star => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => {
                                return Ok(EvalResult::Number(left * right));
                            },
                            (EvalResult::Identifier(left), EvalResult::Identifier(right)) => {
                                let left = self.get_variable_value(left).unwrap();
                                let right = self.get_variable_value(right).unwrap();
                                match (left, right) {
                                    (Value::Number(left), Value::Number(right)) => {
                                        return Ok(EvalResult::Number(left * right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::Number(right)) => {
                                let left = self.get_variable_value(left).unwrap();
                                match left {
                                    Value::Number(left) => {
                                        return Ok(EvalResult::Number(left * right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Number(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap();
                                match right {
                                    Value::Number(right) => {
                                        return Ok(EvalResult::Number(left * right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            _ => return Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::Mod => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => {
                                return Ok(EvalResult::Number(left % right));
                            },
                            (EvalResult::Identifier(left), EvalResult::Identifier(right)) => {
                                let left = self.get_variable_value(left).unwrap();
                                let right = self.get_variable_value(right).unwrap();
                                match (left, right) {
                                    (Value::Number(left), Value::Number(right)) => {
                                        return Ok(EvalResult::Number(left % right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::Number(right)) => {
                                let left = self.get_variable_value(left).unwrap();
                                match left {
                                    Value::Number(left) => {
                                        return Ok(EvalResult::Number(left % right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Number(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap();
                                match right {
                                    Value::Number(right) => {
                                        return Ok(EvalResult::Number(left % right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            _ => return Err(EvalError::InvalidDataType),
                        }
                    },
                    // Comparison
                    BinaryOperator::DoubleEqual => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => {
                                return Ok(EvalResult::Boolean(left == right));
                            },
                            (EvalResult::String(left), EvalResult::String(right)) => {
                                return Ok(EvalResult::Boolean(left == right));
                            },
                            (EvalResult::Boolean(left), EvalResult::Boolean(right)) => {
                                return Ok(EvalResult::Boolean(left == right));
                            },
                            (EvalResult::Identifier(left), EvalResult::Identifier(right)) => {
                                let left = self.get_variable_value(left).unwrap();
                                let right = self.get_variable_value(right).unwrap();
                                match (left, right) {
                                    (Value::Number(left), Value::Number(right)) => {
                                        return Ok(EvalResult::Boolean(left == right));
                                    },
                                    (Value::String(left), Value::String(right)) => {
                                        return Ok(EvalResult::Boolean(left == right));
                                    },
                                    (Value::Boolean(left), Value::Boolean(right)) => {
                                        return Ok(EvalResult::Boolean(left == right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::Number(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                match left {
                                    Value::Number(left) => {
                                        return Ok(EvalResult::Boolean(left == right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::String(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                match left {
                                    Value::String(left) => {
                                        return Ok(EvalResult::Boolean(left == right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::Boolean(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                match left {
                                    Value::Boolean(left) => {
                                        return Ok(EvalResult::Boolean(left == right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Number(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap().clone();
                                match right {
                                    Value::Number(right) => {
                                        return Ok(EvalResult::Boolean(left == right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::String(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap().clone();
                                match right {
                                    Value::String(right) => {
                                        return Ok(EvalResult::Boolean(left == right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Boolean(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap().clone();
                                match right {
                                    Value::Boolean(right) => {
                                        return Ok(EvalResult::Boolean(left == right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            _ => return Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::GreaterThan => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => {
                                return Ok(EvalResult::Boolean(left > right));
                            },
                            (EvalResult::Identifier(left), EvalResult::Identifier(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                let right = self.get_variable_value(right).unwrap().clone();
                                match (left, right) {
                                    (Value::Number(left), Value::Number(right)) => {
                                        return Ok(EvalResult::Boolean(left > right));
                                    }
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::Number(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                match left {
                                    Value::Number(left) => {
                                        return Ok(EvalResult::Boolean(left > right));
                                    }
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Number(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap().clone();
                                match right {
                                    Value::Number(right) => {
                                        return Ok(EvalResult::Boolean(left > right));
                                    }
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            _ => return Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::GreaterThanEqual => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => {
                                return Ok(EvalResult::Boolean(left >= right));
                            },
                            (EvalResult::Identifier(left), EvalResult::Identifier(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                let right = self.get_variable_value(right).unwrap().clone();
                                match (left, right) {
                                    (Value::Number(left), Value::Number(right)) => {
                                        return Ok(EvalResult::Boolean(left >= right));
                                    }
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::Number(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                match left {
                                    Value::Number(left) => {
                                        return Ok(EvalResult::Boolean(left >= right));
                                    }
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Number(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap().clone();
                                match right {
                                    Value::Number(right) => {
                                        return Ok(EvalResult::Boolean(left >= right));
                                    }
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            _ => return Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::LessThan => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => {
                                return Ok(EvalResult::Boolean(left < right));
                            },
                            (EvalResult::Identifier(left), EvalResult::Identifier(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                let right = self.get_variable_value(right).unwrap().clone();
                                match (left, right) {
                                    (Value::Number(left), Value::Number(right)) => {
                                        return Ok(EvalResult::Boolean(left < right));
                                    }
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::Number(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                match left {
                                    Value::Number(left) => {
                                        return Ok(EvalResult::Boolean(left < right));
                                    }
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Number(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap().clone();
                                match right {
                                    Value::Number(right) => {
                                        return Ok(EvalResult::Boolean(left < right));
                                    }
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            _ => return Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::LessThanEqual => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => {
                                return Ok(EvalResult::Boolean(left <= right));
                            },
                            (EvalResult::Identifier(left), EvalResult::Identifier(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                let right = self.get_variable_value(right).unwrap().clone();
                                match (left, right) {
                                    (Value::Number(left), Value::Number(right)) => {
                                        return Ok(EvalResult::Boolean(left <= right));
                                    }
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::Number(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                match left {
                                    Value::Number(left) => {
                                        return Ok(EvalResult::Boolean(left <= right));
                                    }
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Number(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap().clone();
                                match right {
                                    Value::Number(right) => {
                                        return Ok(EvalResult::Boolean(left <= right));
                                    }
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            _ => return Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::NotEqual => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => {
                                return Ok(EvalResult::Boolean(left != right));
                            },
                            (EvalResult::String(left), EvalResult::String(right)) => {
                                return Ok(EvalResult::Boolean(left != right));
                            },
                            (EvalResult::Boolean(left), EvalResult::Boolean(right)) => {
                                return Ok(EvalResult::Boolean(left != right));
                            },
                            (EvalResult::Identifier(left), EvalResult::Identifier(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                let right = self.get_variable_value(right).unwrap().clone();
                                match (left, right) {
                                    (Value::Number(left), Value::Number(right)) => {
                                        return Ok(EvalResult::Boolean(left != right));
                                    },
                                    (Value::String(left), Value::String(right)) => {
                                        return Ok(EvalResult::Boolean(left != right));
                                    },
                                    (Value::Boolean(left), Value::Boolean(right)) => {
                                        return Ok(EvalResult::Boolean(left != right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::Number(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                match left {
                                    Value::Number(left) => {
                                        return Ok(EvalResult::Boolean(left != right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::String(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                match left {
                                    Value::String(left) => {
                                        return Ok(EvalResult::Boolean(left != right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Identifier(left), EvalResult::Boolean(right)) => {
                                let left = self.get_variable_value(left).unwrap().clone();
                                match left {
                                    Value::Boolean(left) => {
                                        return Ok(EvalResult::Boolean(left != right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Number(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap().clone();
                                match right {
                                    Value::Number(right) => {
                                        return Ok(EvalResult::Boolean(left != right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::String(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap().clone();
                                match right {
                                    Value::String(right) => {
                                        return Ok(EvalResult::Boolean(left != right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            (EvalResult::Boolean(left), EvalResult::Identifier(right)) => {
                                let right = self.get_variable_value(right).unwrap().clone();
                                match right {
                                    Value::Boolean(right) => {
                                        return Ok(EvalResult::Boolean(left != right));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            _ => return Err(EvalError::InvalidDataType),
                        }
                    },
                }
            },
            Expression::UnaryOp(op, right) => {
                let right = self.eval_expression(*right)?;
                match op {
                    UnaryOperator::Negate => {
                        match right {
                            EvalResult::Number(number) => {
                                return Ok(EvalResult::Number(-number));
                            },
                            EvalResult::Identifier(identifier) => {
                                let value = self.get_variable_value(identifier).unwrap().clone();
                                match value {
                                    Value::Number(number) => {
                                        return Ok(EvalResult::Number(-number));
                                    },
                                    _ => return Err(EvalError::InvalidDataType),
                                }
                            },
                            _ => return Err(EvalError::InvalidDataType),
                        }
                    }
                }
            },
            Expression::Literal(literal) => {
                match literal {
                    Literal::Number(number) => {
                        return Ok(EvalResult::Number(number));
                    }
                    Literal::String(string) => {
                        return Ok(EvalResult::String(string));
                    }
                    Literal::Boolean(boolean) => {
                        return Ok(EvalResult::Boolean(boolean));
                    }
                }
            }
            Expression::Identifier(identifier) => {
                println!("{:?}", self);
                let value = self.get_variable_value(identifier);
                if value.is_none() {
                    return Err(EvalError::InvalidExpression);
                }
                match value.unwrap().clone() {
                    Value::Number(number) => {
                        return Ok(EvalResult::Number(number));
                    },
                    Value::String(string) => {
                        return Ok(EvalResult::String(string));
                    },
                    Value::Boolean(boolean) => {
                        return Ok(EvalResult::Boolean(boolean));
                    },
                    _ => return Err(EvalError::InvalidDataType),
                }
            }
        }
    }
}