use rslua::ast::BinOp;

use crate::{error::Error, Binary, ConstValue};

pub fn int(
    _binary: &mut Binary,
    left: i64,
    right: ConstValue,
    op: BinOp,
) -> Result<ConstValue, Error> {
    match op {
        BinOp::Add => match right {
            ConstValue::Int { value } => Ok(ConstValue::Int {
                value: left + value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Int { value: left },
                right,
                op,
            )),
        },
        BinOp::Minus => match right {
            ConstValue::Int { value } => Ok(ConstValue::Int {
                value: left - value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Int { value: left },
                right,
                op,
            )),
        },
        BinOp::Mul => match right {
            ConstValue::Int { value } => Ok(ConstValue::Int {
                value: left * value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Int { value: left },
                right,
                op,
            )),
        },
        BinOp::Mod => match right {
            ConstValue::Int { value } => Ok(ConstValue::Int {
                value: left % value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Int { value: left },
                right,
                op,
            )),
        },
        BinOp::Pow => match right {
            ConstValue::Int { value } => Ok(ConstValue::Int {
                value: left.pow(value as u32),
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Int { value: left },
                right,
                op,
            )),
        },
        BinOp::Div => match right {
            ConstValue::Int { value } => Ok(ConstValue::Int {
                value: left / value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Int { value: left },
                right,
                op,
            )),
        },
        BinOp::IDiv => match right {
            ConstValue::Int { value } => Ok(ConstValue::Int {
                value: left / value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Int { value: left },
                right,
                op,
            )),
        },
        BinOp::And | BinOp::BAnd => match right {
            ConstValue::Int { value } => Ok(ConstValue::Int {
                value: left & value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Int { value: left },
                right,
                op,
            )),
        },
        BinOp::Or | BinOp::BOr => match right {
            ConstValue::Int { value } => Ok(ConstValue::Int {
                value: left | value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Int { value: left },
                right,
                op,
            )),
        },
        BinOp::BXor => match right {
            ConstValue::Int { value } => Ok(ConstValue::Int {
                value: left ^ value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Int { value: left },
                right,
                op,
            )),
        },
        BinOp::Shl => match right {
            ConstValue::Int { value } => Ok(ConstValue::Int {
                value: left << value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Int { value: left },
                right,
                op,
            )),
        },
        BinOp::Shr => match right {
            ConstValue::Int { value } => Ok(ConstValue::Int {
                value: left >> value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Int { value: left },
                right,
                op,
            )),
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
) -> Result<ConstValue, Error> {
    match op {
        BinOp::Add => match right {
            ConstValue::UInt { value } => Ok(ConstValue::UInt {
                value: left + value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::UInt { value: left },
                right,
                op,
            )),
        },
        BinOp::Minus => match right {
            ConstValue::UInt { value } => Ok(ConstValue::UInt {
                value: left - value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::UInt { value: left },
                right,
                op,
            )),
        },
        BinOp::Mul => match right {
            ConstValue::UInt { value } => Ok(ConstValue::UInt {
                value: left * value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::UInt { value: left },
                right,
                op,
            )),
        },
        BinOp::Mod => match right {
            ConstValue::UInt { value } => Ok(ConstValue::UInt {
                value: left % value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::UInt { value: left },
                right,
                op,
            )),
        },
        BinOp::Pow => match right {
            ConstValue::UInt { value } => Ok(ConstValue::UInt {
                value: left.pow(value as u32),
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::UInt { value: left },
                right,
                op,
            )),
        },
        BinOp::Div => match right {
            ConstValue::UInt { value } => Ok(ConstValue::UInt {
                value: left / value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::UInt { value: left },
                right,
                op,
            )),
        },
        BinOp::IDiv => match right {
            ConstValue::UInt { value } => Ok(ConstValue::UInt {
                value: left / value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::UInt { value: left },
                right,
                op,
            )),
        },
        BinOp::And | BinOp::BAnd => match right {
            ConstValue::UInt { value } => Ok(ConstValue::UInt {
                value: left & value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::UInt { value: left },
                right,
                op,
            )),
        },
        BinOp::Or | BinOp::BOr => match right {
            ConstValue::UInt { value } => Ok(ConstValue::UInt {
                value: left | value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::UInt { value: left },
                right,
                op,
            )),
        },
        BinOp::BXor => match right {
            ConstValue::UInt { value } => Ok(ConstValue::UInt {
                value: left ^ value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::UInt { value: left },
                right,
                op,
            )),
        },
        BinOp::Shl => match right {
            ConstValue::UInt { value } => Ok(ConstValue::UInt {
                value: left << value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::UInt { value: left },
                right,
                op,
            )),
        },
        BinOp::Shr => match right {
            ConstValue::UInt { value } => Ok(ConstValue::UInt {
                value: left >> value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::UInt { value: left },
                right,
                op,
            )),
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
) -> Result<ConstValue, Error> {
    match op {
        BinOp::Add => match right {
            ConstValue::Float { value } => Ok(ConstValue::Float {
                value: left + value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Float { value: left },
                right,
                op,
            )),
        },
        BinOp::Minus => match right {
            ConstValue::Float { value } => Ok(ConstValue::Float {
                value: left - value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Float { value: left },
                right,
                op,
            )),
        },
        BinOp::Mul => match right {
            ConstValue::Float { value } => Ok(ConstValue::Float {
                value: left * value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Float { value: left },
                right,
                op,
            )),
        },
        BinOp::Mod => match right {
            ConstValue::Float { value } => Ok(ConstValue::Float {
                value: left % value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Float { value: left },
                right,
                op,
            )),
        },
        BinOp::Pow => match right {
            ConstValue::Int { value } => Ok(ConstValue::Float {
                value: left.powi(value as i32),
            }),
            ConstValue::Float { value } => Ok(ConstValue::Float {
                value: left.powf(value),
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Float { value: left },
                right,
                op,
            )),
        },
        BinOp::Div => match right {
            ConstValue::Float { value } => Ok(ConstValue::Float {
                value: left / value,
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Float { value: left },
                right,
                op,
            )),
        },
        BinOp::IDiv => match right {
            ConstValue::Float { value } => Ok(ConstValue::Float {
                value: (left / value).floor(),
            }),
            ConstValue::Const { .. } => unimplemented!("Constexpr"),
            _ => Err(Error::InvalidBinaryOperator(
                ConstValue::Float { value: left },
                right,
                op,
            )),
        },
        BinOp::Concat => todo!(),
        BinOp::Ne => todo!(),
        BinOp::Eq => todo!(),
        BinOp::Lt => todo!(),
        BinOp::Le => todo!(),
        BinOp::Gt => todo!(),
        BinOp::Ge => todo!(),
        BinOp::None => todo!(),
        _ => Err(Error::InvalidBinaryOperator(
            ConstValue::Float { value: left },
            right,
            op,
        )),
    }
}
