pub mod error;
pub mod root;
pub mod root_funcs;

use std::io::Write;

use anyhow::Result;
use rslua::ast::Stat;

#[derive(Debug)]
pub struct Binary {
    pub statics: Vec<Constant>,
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
pub struct Constant {
    pub name: String,
    pub value: ConstValue,
}

// Use direct values as there are no ranges provided
#[derive(Clone, Debug)]
pub enum ConstValue {
    Int { value: i64 },
    UInt { value: u64 },
    Float { value: f64 },
    String { value: String },
    Buffer { size: usize },
    BufferOffset { index: usize, offset: i64 },
    Const { name: String },
}

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub param_count: usize,
}

pub fn parse(source: &str, _outfile: &mut impl Write) -> Result<()> {
    let mut binary = Binary::default();
    let mut lexer = rslua::lexer::Lexer::new();
    let r1 = lexer.run(source).unwrap();
    let r2 = rslua::parser::Parser::new().run(r1).unwrap();
    println!("{r2:#?}");
    for stat in r2.stats {
        match stat.stat {
            Stat::IfStat(_) => unimplemented!(),
            Stat::WhileStat(_) => unimplemented!(),
            Stat::DoBlock(_) => unimplemented!(),
            Stat::ForStat(_) => unimplemented!(),
            Stat::RepeatStat(_) => unimplemented!(),
            Stat::FuncStat(_) => unimplemented!(),
            Stat::LocalStat(_) => unimplemented!(),
            Stat::LabelStat(_) => unimplemented!(),
            Stat::RetStat(_) => unimplemented!(),
            Stat::BreakStat(_) => unimplemented!(),
            Stat::GotoStat(_) => unimplemented!(),
            Stat::AssignStat(assign) => self::root::parse_assign(&mut binary, assign, stat.source)?,
            Stat::CallStat(_) => unimplemented!(),
            Stat::CommentStat(_) => {}
        }
    }
    println!("{binary:#?}");
    Ok(())
}

// fn parse_const_assignment(binary: &mut Binary, assignment: &Assignment) -> Result<()> {
//     if assignment.variables().is_empty() || assignment.expressions().is_empty() {
//         panic!("Invalid assignment");
//     }
//     if assignment.variables().len() > 1 || assignment.expressions().len() > 1 {
//         unimplemented!("Multiple assignments are unsupported");
//     }
//     let var = assignment.variables().iter().last().unwrap();
//     let expr = assignment.expressions().iter().last().unwrap();
//     let name = match var {
//         Var::Expression(_) => panic!("Expression assignment is unsupported"),
//         Var::Name(name_token) => name_token.token().to_string(),
//         _ => unreachable!(),
//     };
//     let value = eval_const_assign_expr(binary, expr)?;
//     println!("'{}'", name);
//     println!("'{:?}'", value);
//     Ok(())
// }

// fn parse_const_call(binary: &mut Binary, call: &FunctionCall) {
//     let Prefix::Name(name) = &call.prefix() else {
//         panic!("Expression");
//     };
//     let Some(func) = ROOT_FUNCS.get(&name.token().to_string()) else {
//         panic!("Constant root-level function '{}' could not be found", call.prefix());
//     };
//     func(binary, call);
// }

// fn parse_function(binary: &mut Binary, func: &FunctionDeclaration) {
//     let name = func.name().to_string();
//     let body = func.body();
//     let mut params = Vec::with_capacity(body.parameters().len());
//     for param in body.parameters() {
//         match param {
//             Parameter::Ellipse(_) => panic!("Varargs are not supported"),
//             Parameter::Name(name) => params.push(name.to_string()),
//             _ => unreachable!(),
//         }
//     }
//     let statements = body.block().stmts();
//     for stmnt in statements {
//         match stmnt {
//             Stmt::Assignment(_) => todo!(),
//             Stmt::Do(_) => todo!(),
//             Stmt::FunctionCall(_) => todo!(),
//             Stmt::FunctionDeclaration(_) => todo!(),
//             Stmt::GenericFor(_) => todo!(),
//             Stmt::If(_) => todo!(),
//             Stmt::LocalAssignment(_) => todo!(),
//             Stmt::LocalFunction(_) => todo!(),
//             Stmt::NumericFor(_) => todo!(),
//             Stmt::Repeat(_) => todo!(),
//             Stmt::While(_) => todo!(),
//             _ => unimplemented!(),
//         }
//     }
//     println!(
//         "Function:
//   name: {name}
//   params: {params:?}"
//     );
//     binary.funcs.push(Func {
//         name,
//         param_count: params.len(),
//     });
// }

// fn eval_const_assign_expr(binary: &mut Binary, expr: &Expression) -> Result<ConstValue> {
//     match expr {
//         Expression::BinaryOperator { lhs, binop, rhs } => {
//             let _lhs = eval_const_assign_expr(binary, lhs)?;
//             let _rhs = eval_const_assign_expr(binary, rhs)?;
//             match binop {
//                 BinOp::And(_) => unimplemented!(),
//                 BinOp::Caret(_) => unimplemented!(),
//                 BinOp::GreaterThan(_) => unimplemented!(),
//                 BinOp::GreaterThanEqual(_) => unimplemented!(),
//                 BinOp::LessThan(_) => unimplemented!(),
//                 BinOp::LessThanEqual(_) => unimplemented!(),
//                 BinOp::Minus(_) => unimplemented!(),
//                 BinOp::Or(_) => unimplemented!(),
//                 BinOp::Percent(_) => unimplemented!(),
//                 BinOp::Plus(_) => unimplemented!(),
//                 BinOp::Slash(_) => unimplemented!(),
//                 BinOp::Star(_) => unimplemented!(),
//                 BinOp::TildeEqual(_) => unimplemented!(),
//                 BinOp::TwoDots(_) => unimplemented!(),
//                 BinOp::TwoEqual(_) => unimplemented!(),
//                 _ => todo!(),
//             }
//         }
//         Expression::Parentheses { .. } => unreachable!("Parentheses"),
//         Expression::UnaryOperator { unop, expression } => {
//             let expr = eval_const_assign_expr(binary, expression)?;
//             match expr {
//                 ConstValue::Int { value } => match unop {
//                     UnOp::Minus(_) => Ok(ConstValue::Int { value: -value }),
//                     UnOp::Not(_) => Ok(ConstValue::Int { value: !value }),
//                     _ => unimplemented!("Hash"),
//                 },
//                 ConstValue::UInt { value } => match unop {
//                     UnOp::Minus(_) => Ok(ConstValue::Int {
//                         value: -(value as i64),
//                     }),
//                     UnOp::Not(_) => Ok(ConstValue::UInt { value: !value }),
//                     _ => unimplemented!("Hash"),
//                 },
//                 ConstValue::Float { value } => match unop {
//                     UnOp::Minus(_) => Ok(ConstValue::Float { value: -value }),
//                     _ => unimplemented!("Hash or not"),
//                 },
//                 ConstValue::Buffer { .. } => panic!("Unary op on buffer"),
//             }
//         }
//         Expression::Value { value } => match value.as_ref() {
//             Value::Function(_) => unimplemented!("Func"),
//             Value::FunctionCall(call) => {
//                 let Prefix::Name(name) = &call.prefix() else {
//                     panic!("Expression");
//                 };
//                 let Some(func) = ROOT_ASSIGN_FUNCS.get(&name.token().to_string()) else {
//                     panic!("Constant root-level assign function '{}' could not be found", call.prefix());
//                 };
//                 func(binary, call)
//             }
//             Value::TableConstructor(_) => unimplemented!("Table"),
//             Value::Number(num) => {
//                 let TokenType::Number { text } = num.token_type() else {
//                     unreachable!();
//                 };
//                 if let Ok(num) = text.parse() {
//                     return Ok(ConstValue::Int { value: num });
//                 }
//                 if let Ok(num) = text.parse() {
//                     return Ok(ConstValue::Float { value: num });
//                 }
//                 panic!("Number");
//             }
//             Value::ParenthesesExpression(_) => unimplemented!("Parentheses"),
//             Value::String(_) => unimplemented!("String"),
//             Value::Symbol(_) => unimplemented!("Symbol"),
//             Value::Var(_) => unimplemented!("Var"),
//             _ => unreachable!(),
//         },
//         _ => unreachable!(),
//     }
// }
