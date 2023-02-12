use crate::context::Value;

pub mod noverify;
pub mod safe;

pub trait Stack {
    fn push(&mut self, value: Value);

    fn pop(&mut self) -> Value;
}
