use anyhow::Result;
use phf::{phf_map, Map};

use crate::lua_testing::{error::Error, Binary, ConstValue};

pub type Func = fn(&mut Binary, &[ConstValue]) -> Result<ConstValue>;

pub const FUNCS: Map<&'static str, Func> = phf_map! {
    "u" => u,
};

fn u(_: &mut Binary, args: &[ConstValue]) -> Result<ConstValue> {
    if args.len() != 1 {
        return Err(Error::InvalidConstCallArgsCount("u".to_string(), 1, args.len()).into());
    }
    let arg = &args[0];
    match arg {
        ConstValue::Int { value } => Ok(ConstValue::UInt {
            value: *value as u64,
        }),
        _ => Err(
            Error::InvalidConstCallArgsTypes("u".to_string(), "int".to_string(), arg.clone())
                .into(),
        ),
    }
}
