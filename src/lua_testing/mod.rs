pub mod error;
pub mod func;
pub mod root;

use std::{collections::HashMap, fmt::Display, io::Write};

use anyhow::Result;
use rslua::ast::Stat;

use crate::lua_testing::error::Error;

#[derive(Debug)]
pub struct Binary {
    pub namespace: String,
    pub statics: Vec<Constant>,
    pub path: HashMap<String, PathNode>,
}

impl Binary {
    pub fn new(namespace: impl Into<String>) -> Self {
        let mut res = Self {
            namespace: namespace.into(),
            statics: Vec::with_capacity(0),
            path: HashMap::with_capacity(1),
        };
        res.path.insert(res.namespace.clone(), PathNode::default());
        res
    }
}

impl Default for Binary {
    fn default() -> Self {
        let mut res = Self {
            namespace: "<anonymous>".to_string(),
            statics: Vec::with_capacity(0),
            path: HashMap::with_capacity(1),
        };
        res.path.insert(res.namespace.clone(), PathNode::default());
        res
    }
}

#[derive(Debug)]
pub struct PathNode {
    pub sub_packages: HashMap<String, PathNode>,
    pub funcs: Vec<PathFunc>,
}

impl Default for PathNode {
    fn default() -> Self {
        Self {
            sub_packages: HashMap::with_capacity(0),
            funcs: Vec::with_capacity(0),
        }
    }
}

#[derive(Debug)]
pub enum PathFunc {
    CompiledFunc {
        name: String,
        args_count: usize,
        code: Vec<u8>,
    },
    UncompiledFunc {
        name: String,
        public: bool,
        args_count: usize,
        body: (),
    },
}

#[derive(Debug)]
pub struct Constant {
    pub name: String,
    pub value: ConstValue,
}

#[derive(Clone, Debug)]
pub enum ConstValue {
    Int { value: i64 },
    UInt { value: u64 },
    Float { value: f64 },
    String { value: String },
    Buffer { size: usize },
    BufferOffset { index: usize, offset: i64 },
    Const { name: String },
}

impl ConstValue {
    pub fn type_(&self) -> ConstType {
        match self {
            ConstValue::Int { .. } => ConstType::Int,
            ConstValue::UInt { .. } => ConstType::UInt,
            ConstValue::Float { .. } => ConstType::Float,
            ConstValue::String { .. } => ConstType::String,
            ConstValue::Buffer { .. } => ConstType::Buffer,
            ConstValue::BufferOffset { .. } => ConstType::BufferOffset,
            ConstValue::Const { .. } => ConstType::Const,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstType {
    Int,
    UInt,
    Float,
    String,
    Buffer,
    BufferOffset,
    Const,
}

impl Display for ConstType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstType::Int => f.write_str("int"),
            ConstType::UInt => f.write_str("uint"),
            ConstType::Float => f.write_str("float"),
            ConstType::String => f.write_str("string"),
            ConstType::Buffer => f.write_str("buffer"),
            ConstType::BufferOffset => f.write_str("buffer-offset"),
            ConstType::Const => f.write_str("const"),
        }
    }
}

pub fn parse(source: &str, _outfile: &mut impl Write) -> Result<(), Error> {
    let mut binary = Binary::default();
    let mut lexer = rslua::lexer::Lexer::new();
    let r1 = lexer.run(source).unwrap();
    let r2 = rslua::parser::Parser::new().run(r1).unwrap();
    println!("{r2:#?}");
    for stat in r2.stats {
        match stat.stat {
            Stat::DoBlock(_) => unimplemented!("DoBlock"),
            Stat::FuncStat(func) => self::func::parse_func(&mut binary, func, stat.source)?,
            Stat::LocalStat(_) => unimplemented!("LocalStat"),
            Stat::AssignStat(assign) => self::root::parse_assign(&mut binary, assign, stat.source)?,
            Stat::CallStat(_) => unimplemented!("CallStat"),
            Stat::CommentStat(_) => {}
            _ => {
                return Err(Error::UnsupportedRootElement(stat.source));
            }
        }
    }
    println!("{binary:#?}");
    Ok(())
}
