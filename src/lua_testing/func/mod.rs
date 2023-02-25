use anyhow::Result;
use rslua::{
    ast::{FuncStat, FuncType, LocalStat, Param, Stat},
    types::Source,
};

use super::{error::Error, Binary, PathFunc, PathNode};

pub struct LocalVariable {
    pub name: String,
}

pub fn parse_func(binary: &mut Binary, func: FuncStat, _source: Source) -> Result<(), Error> {
    let mut package = binary
        .path
        .get_mut(&binary.namespace)
        .expect("Project namespace");
    let FuncStat {
        func_type,
        mut func_name,
        body,
    } = func;
    let name;
    if func_name.method.is_none() {
        if func_name.fields.len() != 1 {
            return Err(Error::InvalidFunctionNameConstruct());
        }
        name = func_name.fields.pop().unwrap();
    } else {
        for sub_package in &func_name.fields {
            if !package.sub_packages.contains_key(sub_package) {
                package
                    .sub_packages
                    .insert(sub_package.clone(), PathNode::default());
            }
            package = package.sub_packages.get_mut(sub_package).unwrap();
        }
        name = func_name.method.unwrap();
    }
    let args_count = body.params.len();
    let mut variables = Vec::with_capacity(body.params.len());
    for param in body.params {
        let Param::Name(param_name) = param else {
            return Err(Error::UnsupportedVarargInFunctionSignature(name));
        };
        variables.push(LocalVariable { name: param_name });
    }
    let block = body.block.stats;
    for stat in block {
        match stat.stat {
            Stat::IfStat(_) => todo!(),
            Stat::WhileStat(_) => todo!(),
            Stat::DoBlock(_) => todo!(),
            Stat::ForStat(_) => todo!(),
            Stat::RepeatStat(_) => todo!(),
            Stat::LocalStat(local) => parse_local(local, stat.source)?,
            Stat::RetStat(_) => todo!(),
            Stat::AssignStat(_) => todo!(),
            Stat::CallStat(_) => todo!(),
            Stat::CommentStat(_) => {}
            _ => return Err(Error::UnsupportedElement(stat.source)),
        }
    }
    package.funcs.push(PathFunc::UncompiledFunc {
        name,
        public: func_type == FuncType::Global,
        args_count,
        body: (),
    });
    Ok(())
}

fn parse_local(assign: LocalStat, source: Source) -> Result<(), Error> {
    if assign.names.len() != assign.exprs.len() {
        return Err(Error::InvalidAssignment(source));
    }
    Ok(())
}
