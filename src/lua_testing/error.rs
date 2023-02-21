use rslua::{
    ast::{BinOp, UnOp},
    types::Source,
};
use thiserror::Error;

use super::ConstValue;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid assignment at: {0:?}")]
    InvalidAssignment(Source),
    #[error("Invalid unary operator '{1:?}' for constant value '{0:?}'")]
    InvalidUnaryOperator(ConstValue, UnOp),
    #[error("Invalid binary operator '{2:?}' for constant value '{0:?}' and '{1:?}'")]
    InvalidBinaryOperator(ConstValue, ConstValue, BinOp),
}
