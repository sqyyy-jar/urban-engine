use std::{env::args, fs};

use context::{noverify::UnsafeContext, Context};
use vmod::util::Util;

pub mod asm;
pub mod context;
pub mod int;
pub mod rt;
pub mod stack;
pub mod vmod;

fn main() {
    let mut file = fs::read(args().nth(1).unwrap()).unwrap();
    let mut ctx = UnsafeContext::new(file.as_mut_ptr() as _, file.as_mut_ptr() as _, 8192);
    ctx.load_vmod(&Util);
    loop {
        ctx.decode_instruction()
    }
}
