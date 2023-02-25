mod bin_op;
pub mod const_funcs;
pub mod core;

use anyhow::Result;
use rslua::{
    ast::{AssignStat, Assignable, Expr, FuncArgs, Suffix, SuffixedExpr, UnOp},
    types::Source,
};

use super::{error::Error, Binary, ConstValue};

pub fn parse_assign(binary: &mut Binary, assign: AssignStat, source: Source) -> Result<(), Error> {
    if assign.left.len() != assign.right.len() {
        return Err(Error::InvalidAssignment(source));
    }
    let len = assign.left.len();
    for i in 0..len {
        let left = &assign.left[i];
        let right = &assign.right[i];
        match left {
            Assignable::Name(name) => {
                let value = resolve_expr(binary, right)?;
                binary.statics.push(super::Constant {
                    name: name.clone(),
                    value,
                });
            }
            Assignable::SuffixedExpr(_) => unimplemented!("SuffixedExpr"),
        }
    }
    Ok(())
}

fn parse_assign_suffixed(binary: &mut Binary, expr: &SuffixedExpr) -> Result<ConstValue, Error> {
    // TODO: Change this to not collect all components
    let mut ident = String::new();
    let mut exprs = None;
    match expr.primary.as_ref() {
        Expr::Name(name) => ident.push_str(name),
        _ => return Err(Error::InvalidConstCall()),
    }
    for suffix in &expr.suffixes {
        if exprs.is_some() {
            return Err(Error::InvalidConstCall());
        }
        match suffix {
            Suffix::Attr(attr) => {
                ident.push('.');
                ident.push_str(attr);
            }
            Suffix::Index(_) => unimplemented!("Index operator"),
            Suffix::Method(method) => {
                ident.push(':');
                ident.push_str(method);
            }
            Suffix::FuncArgs(args) => match args {
                FuncArgs::Exprs(value) => exprs = Some(value),
                FuncArgs::Table(_) => unimplemented!("Table args call"),
                FuncArgs::String(_) => unimplemented!("String args call"),
            },
        }
    }
    let Some(exprs) = exprs else {
        return Err(Error::InvalidConstCall());
    };
    let mut args = Vec::with_capacity(exprs.len());
    for expr in exprs {
        args.push(resolve_expr(binary, expr)?);
    }
    let Some(func) = const_funcs::FUNCS.get(&ident) else {
        return Err(Error::InvalidConstCallUnknown(ident));
    };
    func(binary, &args)
}

fn resolve_expr(binary: &mut Binary, expr: &Expr) -> Result<ConstValue, Error> {
    match expr {
        Expr::Nil => Ok(ConstValue::UInt { value: 0 }),
        Expr::True => Ok(ConstValue::UInt { value: u64::MAX }),
        Expr::False => Ok(ConstValue::UInt { value: 0 }),
        Expr::VarArg => unimplemented!("VarArg"),
        Expr::Float(value) => Ok(ConstValue::Float { value: *value }),
        Expr::Int(value) => Ok(ConstValue::Int { value: *value }),
        Expr::String(value) => Ok(ConstValue::String {
            value: value.clone(),
        }),
        Expr::Name(name) => Ok(ConstValue::Const { name: name.clone() }),
        Expr::ParenExpr(expr) => resolve_expr(binary, expr),
        Expr::FuncBody(_) => unimplemented!("FuncBody"),
        Expr::Table(_) => unimplemented!("Table"),
        Expr::BinExpr(bin_expr) => {
            let op = bin_expr.op;
            let left = resolve_expr(binary, &bin_expr.left)?;
            let right = resolve_expr(binary, &bin_expr.right)?;
            match left {
                ConstValue::Int { value } => self::bin_op::int(binary, value, right, op),
                ConstValue::UInt { value } => self::bin_op::uint(binary, value, right, op),
                ConstValue::Float { value } => self::bin_op::float(binary, value, right, op),
                ConstValue::String { value: _ } => unimplemented!("String binop"),
                ConstValue::Buffer { size: _ } => unimplemented!("Buffer binop"),
                ConstValue::BufferOffset {
                    index: _,
                    offset: _,
                } => unimplemented!("Buffer offset binop"),
                ConstValue::Const { name: _ } => unimplemented!("Constant"),
            }
        }
        Expr::UnExpr(un_expr) => {
            let expr = resolve_expr(binary, &un_expr.expr)?;
            match expr {
                ConstValue::Int { value } => match un_expr.op {
                    UnOp::Minus => Ok(ConstValue::Int { value: -value }),
                    UnOp::Not | UnOp::BNot => Ok(ConstValue::Int { value: !value }),
                    UnOp::None => Ok(ConstValue::Int { value }),
                    op => Err(Error::InvalidUnaryOperator(expr, op)),
                },
                ConstValue::UInt { value } => match un_expr.op {
                    UnOp::Minus => Ok(ConstValue::Int {
                        value: -(value as i64),
                    }),
                    UnOp::Not | UnOp::BNot => Ok(ConstValue::UInt { value: !value }),
                    UnOp::None => Ok(ConstValue::UInt { value }),
                    op => Err(Error::InvalidUnaryOperator(expr, op)),
                },
                ConstValue::Float { value } => match un_expr.op {
                    UnOp::Minus => Ok(ConstValue::Float { value: -value }),
                    UnOp::None => Ok(ConstValue::Float { value }),
                    op => Err(Error::InvalidUnaryOperator(expr, op)),
                },
                ConstValue::String { .. } => Err(Error::InvalidUnaryOperator(expr, un_expr.op)),
                ConstValue::Buffer { .. } => Err(Error::InvalidUnaryOperator(expr, un_expr.op)),
                ConstValue::BufferOffset { .. } => {
                    Err(Error::InvalidUnaryOperator(expr, un_expr.op))
                }
                ConstValue::Const { .. } => Err(Error::InvalidUnaryOperator(expr, un_expr.op)),
            }
        }
        Expr::SuffixedExpr(expr) => parse_assign_suffixed(binary, expr),
    }
}
