pub mod root_funcs;

use std::io::{Error, ErrorKind, Result, Write};

use full_moon::{
    ast::{
        Assignment, BinOp, Expression, FunctionCall, FunctionDeclaration, Parameter, Prefix, Stmt,
        UnOp, Value, Var,
    },
    tokenizer::TokenType,
};

use self::root_funcs::{ROOT_ASSIGN_FUNCS, ROOT_FUNCS};

#[derive(Debug)]
pub struct Binary {
    pub statics: Vec<ConstValue>,
    pub funcs: Vec<Func>,
}

impl Default for Binary {
    fn default() -> Self {
        Self {
            statics: Vec::with_capacity(0),
            funcs: Vec::with_capacity(0),
        }
    }
}

#[derive(Debug)]
pub enum ConstValue {
    Int { value: i64 },
    UInt { value: u64 },
    Float { value: f64 },
    Buffer { size: usize },
}

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub param_count: usize,
}

pub fn parse(source: &str, _outfile: &mut impl Write) -> Result<()> {
    let result = full_moon::parse(source);
    if let Err(err) = result {
        return Err(Error::new(ErrorKind::Other, err));
    }
    let mut binary = Binary::default();
    for element in result.unwrap().nodes().stmts() {
        match element {
            Stmt::Assignment(assignment) => parse_const_assignment(&mut binary, assignment)?,
            Stmt::FunctionCall(call) => parse_const_call(&mut binary, call),
            Stmt::FunctionDeclaration(func) => parse_function(&mut binary, func),
            Stmt::LocalAssignment(_) => unimplemented!("Local elements"),
            Stmt::LocalFunction(_) => unimplemented!("Local elements"),
            _ => panic!("Unsupported element at root level: {element}"),
        }
    }
    Ok(())
}

fn parse_const_assignment(binary: &mut Binary, assignment: &Assignment) -> Result<()> {
    if assignment.variables().is_empty() || assignment.expressions().is_empty() {
        panic!("Invalid assignment");
    }
    if assignment.variables().len() > 1 || assignment.expressions().len() > 1 {
        unimplemented!("Multiple assignments are unsupported");
    }
    let var = assignment.variables().iter().last().unwrap();
    let expr = assignment.expressions().iter().last().unwrap();
    let name = match var {
        Var::Expression(_) => panic!("Expression assignment is unsupported"),
        Var::Name(name_token) => name_token.token().to_string(),
        _ => unreachable!(),
    };
    let value = eval_const_assign_expr(binary, expr)?;
    println!("'{}'", name);
    println!("'{:?}'", value);
    Ok(())
}

fn parse_const_call(binary: &mut Binary, call: &FunctionCall) {
    let Prefix::Name(name) = &call.prefix() else {
        panic!("Expression");
    };
    let Some(func) = ROOT_FUNCS.get(&name.token().to_string()) else {
        panic!("Constant root-level function '{}' could not be found", call.prefix());
    };
    func(binary, call);
}

fn parse_function(binary: &mut Binary, func: &FunctionDeclaration) {
    let name = func.name().to_string();
    let body = func.body();
    let mut params = Vec::with_capacity(body.parameters().len());
    for param in body.parameters() {
        match param {
            Parameter::Ellipse(_) => panic!("Varargs are not supported"),
            Parameter::Name(name) => params.push(name.to_string()),
            _ => unreachable!(),
        }
    }
    let statements = body.block().stmts();
    for stmnt in statements {
        match stmnt {
            Stmt::Assignment(_) => todo!(),
            Stmt::Do(_) => todo!(),
            Stmt::FunctionCall(_) => todo!(),
            Stmt::FunctionDeclaration(_) => todo!(),
            Stmt::GenericFor(_) => todo!(),
            Stmt::If(_) => todo!(),
            Stmt::LocalAssignment(_) => todo!(),
            Stmt::LocalFunction(_) => todo!(),
            Stmt::NumericFor(_) => todo!(),
            Stmt::Repeat(_) => todo!(),
            Stmt::While(_) => todo!(),
            _ => unimplemented!(),
        }
    }
    println!(
        "Function: 
  name: {name}
  params: {params:?}"
    );
    binary.funcs.push(Func {
        name,
        param_count: params.len(),
    });
}

fn eval_const_assign_expr(binary: &mut Binary, expr: &Expression) -> Result<ConstValue> {
    match expr {
        Expression::BinaryOperator { lhs, binop, rhs } => {
            let _lhs = eval_const_assign_expr(binary, lhs)?;
            let _rhs = eval_const_assign_expr(binary, rhs)?;
            match binop {
                BinOp::And(_) => unimplemented!(),
                BinOp::Caret(_) => unimplemented!(),
                BinOp::GreaterThan(_) => unimplemented!(),
                BinOp::GreaterThanEqual(_) => unimplemented!(),
                BinOp::LessThan(_) => unimplemented!(),
                BinOp::LessThanEqual(_) => unimplemented!(),
                BinOp::Minus(_) => unimplemented!(),
                BinOp::Or(_) => unimplemented!(),
                BinOp::Percent(_) => unimplemented!(),
                BinOp::Plus(_) => unimplemented!(),
                BinOp::Slash(_) => unimplemented!(),
                BinOp::Star(_) => unimplemented!(),
                BinOp::TildeEqual(_) => unimplemented!(),
                BinOp::TwoDots(_) => unimplemented!(),
                BinOp::TwoEqual(_) => unimplemented!(),
                _ => todo!(),
            }
        }
        Expression::Parentheses { .. } => unreachable!("Parentheses"),
        Expression::UnaryOperator { unop, expression } => {
            let expr = eval_const_assign_expr(binary, expression)?;
            match expr {
                ConstValue::Int { value } => match unop {
                    UnOp::Minus(_) => Ok(ConstValue::Int { value: -value }),
                    UnOp::Not(_) => Ok(ConstValue::Int { value: !value }),
                    _ => unimplemented!("Hash"),
                },
                ConstValue::UInt { value } => match unop {
                    UnOp::Minus(_) => Ok(ConstValue::Int {
                        value: -(value as i64),
                    }),
                    UnOp::Not(_) => Ok(ConstValue::UInt { value: !value }),
                    _ => unimplemented!("Hash"),
                },
                ConstValue::Float { value } => match unop {
                    UnOp::Minus(_) => Ok(ConstValue::Float { value: -value }),
                    _ => unimplemented!("Hash or not"),
                },
                ConstValue::Buffer { .. } => panic!("Unary op on buffer"),
            }
        }
        Expression::Value { value } => match value.as_ref() {
            Value::Function(_) => unimplemented!("Func"),
            Value::FunctionCall(call) => {
                let Prefix::Name(name) = &call.prefix() else {
                    panic!("Expression");
                };
                let Some(func) = ROOT_ASSIGN_FUNCS.get(&name.token().to_string()) else {
                    panic!("Constant root-level assign function '{}' could not be found", call.prefix());
                };
                func(binary, call)
            }
            Value::TableConstructor(_) => unimplemented!("Table"),
            Value::Number(num) => {
                let TokenType::Number { text } = num.token_type() else {
                    unreachable!();
                };
                if let Ok(num) = text.parse() {
                    return Ok(ConstValue::Int { value: num });
                }
                if let Ok(num) = text.parse() {
                    return Ok(ConstValue::Float { value: num });
                }
                panic!("Number");
            }
            Value::ParenthesesExpression(_) => unimplemented!("Parentheses"),
            Value::String(_) => unimplemented!("String"),
            Value::Symbol(_) => unimplemented!("Symbol"),
            Value::Var(_) => unimplemented!("Var"),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}