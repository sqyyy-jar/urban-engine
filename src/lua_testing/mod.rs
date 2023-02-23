pub mod error;
pub mod root;

use std::{fmt::Display, io::Write};

use anyhow::Result;
use rslua::ast::Stat;

#[derive(Debug)]
pub struct Binary {
    pub statics: Vec<Constant>,
    pub funcs: Vec<Func>,
}

impl Default for Binary {
    fn default() -> Self {
        Self {
            statics: Vec::with_capacity(0),
            funcs: Vec::with_capacity(0),
        }
    }
}

#[derive(Debug)]
pub struct Constant {
    pub name: String,
    pub value: ConstValue,
}

// Use direct values as there are no ranges provided
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

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub param_count: usize,
}

pub fn parse(source: &str, _outfile: &mut impl Write) -> Result<()> {
    let mut binary = Binary::default();
    let mut lexer = rslua::lexer::Lexer::new();
    let r1 = lexer.run(source).unwrap();
    let r2 = rslua::parser::Parser::new().run(r1).unwrap();
    println!("{r2:#?}");
    for stat in r2.stats {
        match stat.stat {
            Stat::IfStat(_) => unimplemented!(),
            Stat::WhileStat(_) => unimplemented!(),
            Stat::DoBlock(_) => unimplemented!(),
            Stat::ForStat(_) => unimplemented!(),
            Stat::RepeatStat(_) => unimplemented!(),
            Stat::FuncStat(_) => unimplemented!(),
            Stat::LocalStat(_) => unimplemented!(),
            Stat::LabelStat(_) => unimplemented!(),
            Stat::RetStat(_) => unimplemented!(),
            Stat::BreakStat(_) => unimplemented!(),
            Stat::GotoStat(_) => unimplemented!(),
            Stat::AssignStat(assign) => self::root::parse_assign(&mut binary, assign, stat.source)?,
            Stat::CallStat(_) => unimplemented!(),
            Stat::CommentStat(_) => {}
        }
    }
    println!("{binary:#?}");
    Ok(())
}
