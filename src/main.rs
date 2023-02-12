use std::{env::args, fs};

use context::{noverify::UnsafeContext, Context};

pub mod asm;
pub mod context;
pub mod int;
pub mod rt;
pub mod stack;

fn main() {
    let mut file = fs::read(args().nth(1).unwrap()).unwrap();
    let mut ctx = UnsafeContext::new(file.as_mut_ptr() as _, file.as_mut_ptr() as _, 8192);
    loop {
        // dbg!(ctx.mem);
        ctx.decode_instruction()
    }
}
