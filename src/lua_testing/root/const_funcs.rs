use anyhow::Result;
use phf::{phf_map, Map};

use crate::lua_testing::{error::Error, Binary, ConstType, ConstValue};

pub type Func = fn(&mut Binary, &[ConstValue]) -> Result<ConstValue>;

pub const FUNCS: Map<&'static str, Func> = phf_map! {
    "u" => u,
    "core:const_alloc" => super::core::const_alloc,
};

pub fn expect_args_len(name: &str, args: &[ConstValue], len: usize) -> Result<()> {
    if args.len() != len {
        return Err(Error::InvalidConstCallArgsCount(name.to_string(), len, args.len()).into());
    }
    Ok(())
}

pub fn expect_args(name: &str, args: &[ConstValue], expected: &[ConstType]) -> Result<()> {
    expect_args_len(name, args, expected.len())?;
    for i in 0..args.len() {
        if args[i].type_() != expected[i] {
            return Err(Error::InvalidConstCallArgsTypes(
                name.to_string(),
                expected[i],
                args[i].type_(),
            )
            .into());
        }
    }
    Ok(())
}

fn u(_: &mut Binary, args: &[ConstValue]) -> Result<ConstValue> {
    expect_args("u", args, &[ConstType::Int])?;
    let ConstValue::Int { value } = &args[0] else {
        unreachable!();
    };
    Ok(ConstValue::UInt {
        value: *value as u64,
    })
}
