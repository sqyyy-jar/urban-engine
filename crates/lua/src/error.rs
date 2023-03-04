use rslua::{
    ast::{BinOp, UnOp},
    types::Source,
};
use thiserror::Error;

use super::{ConstType, ConstValue};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unsupported root element at: {0:?}")]
    UnsupportedRootElement(Source),
    #[error("Unsupported element at: {0:?}")]
    UnsupportedElement(Source),
    #[error("Unsupported vararg in signature of function '{0}'")]
    UnsupportedVarargInFunctionSignature(String),
    #[error("Invalid assignment at: {0:?}")]
    InvalidAssignment(Source),
    #[error("Invalid unary operator '{1:?}' for constant value '{0:?}'")]
    InvalidUnaryOperator(ConstValue, UnOp),
    #[error("Invalid binary operator '{2:?}' for constant value '{0:?}' and '{1:?}'")]
    InvalidBinaryOperator(ConstValue, ConstValue, BinOp),
    #[error("Invalid constant call")]
    InvalidConstCall(),
    #[error("Invalid constant call to unknown '{0}'")]
    InvalidConstCallUnknown(String),
    #[error(
        "Invalid amount of passed arguments in constant call to '{0}'. Expected {1} but got {2}"
    )]
    InvalidConstCallArgsCount(String, usize, usize),
    #[error("Invalid types passed in constant call to '{0}'. Expected {1} but got {2}")]
    InvalidConstCallArgsTypes(String, ConstType, ConstType),
    #[error("Invalid function name construct. Expected something like 'a.b:c', 'a:b' or just 'a'")]
    InvalidFunctionNameConstruct(),
    #[error("Invalid function name: '{0}'")]
    InvalidFunctionName(String),
}
