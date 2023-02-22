use anyhow::Result;
use rslua::{
    ast::{AssignStat, Assignable, Expr, SuffixedExpr, UnOp},
    types::Source,
};

use super::{error::Error, Binary, ConstValue};

pub fn parse_assign(binary: &mut Binary, assign: AssignStat, source: Source) -> Result<()> {
    if assign.left.len() != assign.right.len() {
        return Err(Error::InvalidAssignment(source).into());
    }
    let len = assign.left.len();
    for i in 0..len {
        let left = &assign.left[i];
        let right = &assign.right[i];
        match left {
            Assignable::Name(name) => {
                let value = parse_assign_name(binary, right)?;
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

fn parse_assign_name(_binary: &mut Binary, expr: &Expr) -> Result<ConstValue> {
    resolve_expr(_binary, expr)
}

fn parse_assign_suffixed(_binary: &mut Binary, _expr: &SuffixedExpr) {
    dbg!(_expr.suffixes.len());
}

fn resolve_expr(binary: &mut Binary, expr: &Expr) -> Result<ConstValue> {
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
                ConstValue::String { value: _ } => todo!(),
                ConstValue::Buffer { size: _ } => todo!(),
                ConstValue::BufferOffset {
                    index: _,
                    offset: _,
                } => todo!(),
                ConstValue::Const { name: _ } => todo!(),
            }
        }
        Expr::UnExpr(un_expr) => {
            let expr = resolve_expr(binary, &un_expr.expr)?;
            match expr {
                ConstValue::Int { value } => match un_expr.op {
                    UnOp::Minus => Ok(ConstValue::Int { value: -value }),
                    UnOp::Not | UnOp::BNot => Ok(ConstValue::Int { value: !value }),
                    UnOp::None => Ok(ConstValue::Int { value }),
                    op => Err(Error::InvalidUnaryOperator(expr, op).into()),
                },
                ConstValue::UInt { value } => match un_expr.op {
                    UnOp::Minus => Ok(ConstValue::Int {
                        value: -(value as i64),
                    }),
                    UnOp::Not | UnOp::BNot => Ok(ConstValue::UInt { value: !value }),
                    UnOp::None => Ok(ConstValue::UInt { value }),
                    op => Err(Error::InvalidUnaryOperator(expr, op).into()),
                },
                ConstValue::Float { value } => match un_expr.op {
                    UnOp::Minus => Ok(ConstValue::Float { value: -value }),
                    UnOp::None => Ok(ConstValue::Float { value }),
                    op => Err(Error::InvalidUnaryOperator(expr, op).into()),
                },
                ConstValue::String { .. } => {
                    Err(Error::InvalidUnaryOperator(expr, un_expr.op).into())
                }
                ConstValue::Buffer { .. } => {
                    Err(Error::InvalidUnaryOperator(expr, un_expr.op).into())
                }
                ConstValue::BufferOffset { .. } => {
                    Err(Error::InvalidUnaryOperator(expr, un_expr.op).into())
                }
                ConstValue::Const { .. } => {
                    Err(Error::InvalidUnaryOperator(expr, un_expr.op).into())
                }
            }
        }
        Expr::SuffixedExpr(expr) => {
            parse_assign_suffixed(binary, expr);
            todo!()
        }
    }
}

mod bin_op {
    use anyhow::Result;
    use rslua::ast::BinOp;

    use crate::lua_testing::{error::Error, Binary, ConstValue};

    pub fn int(
        _binary: &mut Binary,
        left: i64,
        right: ConstValue,
        op: BinOp,
    ) -> Result<ConstValue> {
        match op {
            BinOp::Add => match right {
                ConstValue::Int { value } => Ok(ConstValue::Int {
                    value: left + value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::Int { value: left }, right, op).into(),
                ),
            },
            BinOp::Minus => match right {
                ConstValue::Int { value } => Ok(ConstValue::Int {
                    value: left - value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::Int { value: left }, right, op).into(),
                ),
            },
            BinOp::Mul => match right {
                ConstValue::Int { value } => Ok(ConstValue::Int {
                    value: left * value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::Int { value: left }, right, op).into(),
                ),
            },
            BinOp::Mod => match right {
                ConstValue::Int { value } => Ok(ConstValue::Int {
                    value: left % value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::Int { value: left }, right, op).into(),
                ),
            },
            BinOp::Pow => match right {
                ConstValue::Int { value } => Ok(ConstValue::Int {
                    value: left.pow(value as u32),
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::Int { value: left }, right, op).into(),
                ),
            },
            BinOp::Div => match right {
                ConstValue::Int { value } => Ok(ConstValue::Int {
                    value: left / value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::Int { value: left }, right, op).into(),
                ),
            },
            BinOp::IDiv => match right {
                ConstValue::Int { value } => Ok(ConstValue::Int {
                    value: left / value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::Int { value: left }, right, op).into(),
                ),
            },
            BinOp::And | BinOp::BAnd => match right {
                ConstValue::Int { value } => Ok(ConstValue::Int {
                    value: left & value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::Int { value: left }, right, op).into(),
                ),
            },
            BinOp::Or | BinOp::BOr => match right {
                ConstValue::Int { value } => Ok(ConstValue::Int {
                    value: left | value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::Int { value: left }, right, op).into(),
                ),
            },
            BinOp::BXor => match right {
                ConstValue::Int { value } => Ok(ConstValue::Int {
                    value: left ^ value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::Int { value: left }, right, op).into(),
                ),
            },
            BinOp::Shl => match right {
                ConstValue::Int { value } => Ok(ConstValue::Int {
                    value: left << value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::Int { value: left }, right, op).into(),
                ),
            },
            BinOp::Shr => match right {
                ConstValue::Int { value } => Ok(ConstValue::Int {
                    value: left >> value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::Int { value: left }, right, op).into(),
                ),
            },
            BinOp::Concat => todo!(),
            BinOp::Ne => todo!(),
            BinOp::Eq => todo!(),
            BinOp::Lt => todo!(),
            BinOp::Le => todo!(),
            BinOp::Gt => todo!(),
            BinOp::Ge => todo!(),
            BinOp::None => todo!(),
        }
    }

    pub fn uint(
        _binary: &mut Binary,
        left: u64,
        right: ConstValue,
        op: BinOp,
    ) -> Result<ConstValue> {
        match op {
            BinOp::Add => match right {
                ConstValue::UInt { value } => Ok(ConstValue::UInt {
                    value: left + value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::UInt { value: left }, right, op)
                        .into(),
                ),
            },
            BinOp::Minus => match right {
                ConstValue::UInt { value } => Ok(ConstValue::UInt {
                    value: left - value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::UInt { value: left }, right, op)
                        .into(),
                ),
            },
            BinOp::Mul => match right {
                ConstValue::UInt { value } => Ok(ConstValue::UInt {
                    value: left * value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::UInt { value: left }, right, op)
                        .into(),
                ),
            },
            BinOp::Mod => match right {
                ConstValue::UInt { value } => Ok(ConstValue::UInt {
                    value: left % value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::UInt { value: left }, right, op)
                        .into(),
                ),
            },
            BinOp::Pow => match right {
                ConstValue::UInt { value } => Ok(ConstValue::UInt {
                    value: left.pow(value as u32),
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::UInt { value: left }, right, op)
                        .into(),
                ),
            },
            BinOp::Div => match right {
                ConstValue::UInt { value } => Ok(ConstValue::UInt {
                    value: left / value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::UInt { value: left }, right, op)
                        .into(),
                ),
            },
            BinOp::IDiv => match right {
                ConstValue::UInt { value } => Ok(ConstValue::UInt {
                    value: left / value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::UInt { value: left }, right, op)
                        .into(),
                ),
            },
            BinOp::And | BinOp::BAnd => match right {
                ConstValue::UInt { value } => Ok(ConstValue::UInt {
                    value: left & value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::UInt { value: left }, right, op)
                        .into(),
                ),
            },
            BinOp::Or | BinOp::BOr => match right {
                ConstValue::UInt { value } => Ok(ConstValue::UInt {
                    value: left | value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::UInt { value: left }, right, op)
                        .into(),
                ),
            },
            BinOp::BXor => match right {
                ConstValue::UInt { value } => Ok(ConstValue::UInt {
                    value: left ^ value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::UInt { value: left }, right, op)
                        .into(),
                ),
            },
            BinOp::Shl => match right {
                ConstValue::UInt { value } => Ok(ConstValue::UInt {
                    value: left << value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::UInt { value: left }, right, op)
                        .into(),
                ),
            },
            BinOp::Shr => match right {
                ConstValue::UInt { value } => Ok(ConstValue::UInt {
                    value: left >> value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => Err(
                    Error::InvalidBinaryOperator(ConstValue::UInt { value: left }, right, op)
                        .into(),
                ),
            },
            BinOp::Concat => todo!(),
            BinOp::Ne => todo!(),
            BinOp::Eq => todo!(),
            BinOp::Lt => todo!(),
            BinOp::Le => todo!(),
            BinOp::Gt => todo!(),
            BinOp::Ge => todo!(),
            BinOp::None => todo!(),
        }
    }

    pub fn float(
        _binary: &mut Binary,
        left: f64,
        right: ConstValue,
        op: BinOp,
    ) -> Result<ConstValue> {
        match op {
            BinOp::Add => match right {
                ConstValue::Float { value } => Ok(ConstValue::Float {
                    value: left + value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => {
                    Err(
                        Error::InvalidBinaryOperator(ConstValue::Float { value: left }, right, op)
                            .into(),
                    )
                }
            },
            BinOp::Minus => match right {
                ConstValue::Float { value } => Ok(ConstValue::Float {
                    value: left - value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => {
                    Err(
                        Error::InvalidBinaryOperator(ConstValue::Float { value: left }, right, op)
                            .into(),
                    )
                }
            },
            BinOp::Mul => match right {
                ConstValue::Float { value } => Ok(ConstValue::Float {
                    value: left * value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => {
                    Err(
                        Error::InvalidBinaryOperator(ConstValue::Float { value: left }, right, op)
                            .into(),
                    )
                }
            },
            BinOp::Mod => match right {
                ConstValue::Float { value } => Ok(ConstValue::Float {
                    value: left % value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => {
                    Err(
                        Error::InvalidBinaryOperator(ConstValue::Float { value: left }, right, op)
                            .into(),
                    )
                }
            },
            BinOp::Pow => match right {
                ConstValue::Int { value } => Ok(ConstValue::Float {
                    value: left.powi(value as i32),
                }),
                ConstValue::Float { value } => Ok(ConstValue::Float {
                    value: left.powf(value),
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => {
                    Err(
                        Error::InvalidBinaryOperator(ConstValue::Float { value: left }, right, op)
                            .into(),
                    )
                }
            },
            BinOp::Div => match right {
                ConstValue::Float { value } => Ok(ConstValue::Float {
                    value: left / value,
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => {
                    Err(
                        Error::InvalidBinaryOperator(ConstValue::Float { value: left }, right, op)
                            .into(),
                    )
                }
            },
            BinOp::IDiv => match right {
                ConstValue::Float { value } => Ok(ConstValue::Float {
                    value: (left / value).floor(),
                }),
                ConstValue::Const { .. } => unimplemented!("Constexpr"),
                _ => {
                    Err(
                        Error::InvalidBinaryOperator(ConstValue::Float { value: left }, right, op)
                            .into(),
                    )
                }
            },
            BinOp::Concat => todo!(),
            BinOp::Ne => todo!(),
            BinOp::Eq => todo!(),
            BinOp::Lt => todo!(),
            BinOp::Le => todo!(),
            BinOp::Gt => todo!(),
            BinOp::Ge => todo!(),
            BinOp::None => todo!(),
            _ => Err(
                Error::InvalidBinaryOperator(ConstValue::Float { value: left }, right, op).into(),
            ),
        }
    }
}
