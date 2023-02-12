use crate::stack::noverify::UnsafeStack;

use super::{immediate, reg, Context, Value};

pub struct UnsafeContext {
    pub code: *const u32,
    pub registers: [Value; 32],
    pub stack: UnsafeStack,
}

impl UnsafeContext {
    pub fn new(code: *const u32, stack_size: usize) -> Self {
        Self {
            code,
            registers: [Value { uint: 0 }; 32],
            stack: UnsafeStack::new(stack_size),
        }
    }
}

impl Context for UnsafeContext {
    #[inline(always)]
    fn fetch_instruction(&mut self) -> u32 {
        unsafe { *self.code }
    }

    #[inline(always)]
    fn advance_counter(&mut self) {
        self.code = unsafe { self.code.add(1) };
    }

    #[inline(always)]
    fn counter(&mut self) -> *const u32 {
        self.code
    }

    #[inline(always)]
    fn set_counter(&mut self, counter: *const u32) {
        self.code = counter;
    }

    #[inline(always)]
    fn add(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint + self.registers[b].uint },
        };
    }

    #[inline(always)]
    fn add_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<16>(insn, 10) as u64;
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint + imm },
        };
    }

    fn addf(&mut self, insn: u32) {
        todo!()
    }

    fn adds(&mut self, insn: u32) {
        todo!()
    }

    fn adds_imm(&mut self, insn: u32) {
        todo!()
    }

    fn and(&mut self, insn: u32) {
        todo!()
    }

    fn b_eq_imm(&mut self, insn: u32) {
        todo!()
    }

    fn b_ge_imm(&mut self, insn: u32) {
        todo!()
    }

    fn b_gt_imm(&mut self, insn: u32) {
        todo!()
    }

    fn b_le_imm(&mut self, insn: u32) {
        todo!()
    }

    fn b_lt_imm(&mut self, insn: u32) {
        todo!()
    }

    fn b_nq_imm(&mut self, insn: u32) {
        todo!()
    }

    fn b_imm(&mut self, insn: u32) {
        todo!()
    }

    fn bl_imm(&mut self, insn: u32) {
        todo!()
    }

    fn br(&mut self, insn: u32) {
        todo!()
    }

    fn brl(&mut self, insn: u32) {
        todo!()
    }

    fn div(&mut self, insn: u32) {
        todo!()
    }

    fn div_imm(&mut self, insn: u32) {
        todo!()
    }

    fn divf(&mut self, insn: u32) {
        todo!()
    }

    fn divs(&mut self, insn: u32) {
        todo!()
    }

    fn divs_imm(&mut self, insn: u32) {
        todo!()
    }

    fn halt(&mut self, insn: u32) {
        todo!()
    }

    fn interrupt_imm(&mut self, insn: u32) {
        todo!()
    }

    fn ldr(&mut self, insn: u32) {
        todo!()
    }

    fn ldr_imm(&mut self, insn: u32) {
        todo!()
    }

    fn mov(&mut self, insn: u32) {
        todo!()
    }

    fn mov_imm(&mut self, insn: u32) {
        todo!()
    }

    fn movs_imm(&mut self, insn: u32) {
        todo!()
    }

    fn mul(&mut self, insn: u32) {
        todo!()
    }

    fn mul_imm(&mut self, insn: u32) {
        todo!()
    }

    fn mulf(&mut self, insn: u32) {
        todo!()
    }

    fn muls(&mut self, insn: u32) {
        todo!()
    }

    fn muls_imm(&mut self, insn: u32) {
        todo!()
    }

    fn nop(&mut self, insn: u32) {
        todo!()
    }

    fn not(&mut self, insn: u32) {
        todo!()
    }

    fn or(&mut self, insn: u32) {
        todo!()
    }

    fn shl(&mut self, insn: u32) {
        todo!()
    }

    fn shr(&mut self, insn: u32) {
        todo!()
    }

    fn shrs(&mut self, insn: u32) {
        todo!()
    }

    fn str(&mut self, insn: u32) {
        todo!()
    }

    fn str_imm(&mut self, insn: u32) {
        todo!()
    }

    fn sub(&mut self, insn: u32) {
        todo!()
    }

    fn sub_imm(&mut self, insn: u32) {
        todo!()
    }

    fn subf(&mut self, insn: u32) {
        todo!()
    }

    fn subs(&mut self, insn: u32) {
        todo!()
    }

    fn subs_imm(&mut self, insn: u32) {
        todo!()
    }

    fn xor(&mut self, insn: u32) {
        todo!()
    }
}
