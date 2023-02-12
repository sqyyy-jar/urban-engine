use asm::{INSN_HALT, INSN_NOP};
use context::{noverify::UnsafeContext, Context};

pub mod asm;
pub mod context;
pub mod rt;
pub mod stack;

fn main() {
    let mut memory = vec![INSN_NOP, INSN_NOP, INSN_NOP, INSN_HALT];
    let mut ctx = UnsafeContext::new(memory.as_mut_ptr(), memory.as_mut_ptr(), 1024);
    loop {
        ctx.decode_instruction()
    }
}
