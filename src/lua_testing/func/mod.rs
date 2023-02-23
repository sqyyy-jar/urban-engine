use anyhow::Result;
use rslua::{ast::FuncStat, types::Source};

use super::{error::Error, Binary};

pub fn parse_func(_binary: &mut Binary, _func: FuncStat, _source: Source) -> Result<(), Error> {
    todo!()
}
