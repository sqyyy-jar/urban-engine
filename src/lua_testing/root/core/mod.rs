use anyhow::Result;

use crate::lua_testing::{Binary, ConstType, ConstValue};

use super::const_funcs::expect_args;

pub(super) fn const_alloc(_: &mut Binary, args: &[ConstValue]) -> Result<ConstValue> {
    expect_args("core:const_alloc", args, &[ConstType::UInt])?;
    let ConstValue::UInt { value } = &args[0] else {
        unreachable!();
    };
    Ok(ConstValue::Buffer {
        size: *value as usize,
    })
}
