use crate::{error::Error, Binary, ConstType, ConstValue};

use super::const_funcs::expect_args;

pub(super) fn const_alloc(_: &mut Binary, args: &[ConstValue]) -> Result<ConstValue, Error> {
    expect_args("core:const_alloc", args, &[ConstType::UInt])?;
    let ConstValue::UInt { value } = &args[0] else {
        unreachable!();
    };
    Ok(ConstValue::Buffer {
        size: *value as usize,
    })
}
