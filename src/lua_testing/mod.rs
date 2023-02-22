pub mod error;
pub mod root;

use std::io::Write;

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
