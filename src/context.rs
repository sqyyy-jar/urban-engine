use crate::{asm::*, stack::Stack};

#[derive(Clone, Copy)]
pub union Value {
    pub uint: u64,
    pub int: u64,
    pub float: f64,
}

pub struct Context {
    pub code: *const u32,
    pub registers: [Value; 32],
    pub stack: Stack,
}

impl Context {
    pub fn new(code: *const u32, stack_size: usize) -> Self {
        Self {
            code,
            registers: [Value { uint: 0 }; 32],
            stack: Stack::new(stack_size),
        }
    }

    pub fn decode_instruction(&mut self) {
        let insn = unsafe { *self.code };
        match insn {
            INSN_ADD..=ENDINSN_ADD => self.add(insn),
            INSN_ADD_IMMEDIATE..=ENDINSN_ADD_IMMEDIATE => self.add_imm(insn),
            INSN_ADDF..=ENDINSN_ADDF => {}
            INSN_ADDS..=ENDINSN_ADDS => {}
            INSN_ADDS_IMMEDIATE..=ENDINSN_ADDS_IMMEDIATE => {}
            INSN_AND..=ENDINSN_AND => {}
            INSN_B_EQ_IMMEDIATE..=ENDINSN_B_EQ_IMMEDIATE => {}
            INSN_B_GE_IMMEDIATE..=ENDINSN_B_GE_IMMEDIATE => {}
            INSN_B_GT_IMMEDIATE..=ENDINSN_B_GT_IMMEDIATE => {}
            INSN_B_LE_IMMEDIATE..=ENDINSN_B_LE_IMMEDIATE => {}
            INSN_B_LT_IMMEDIATE..=ENDINSN_B_LT_IMMEDIATE => {}
            INSN_B_NQ_IMMEDIATE..=ENDINSN_B_NQ_IMMEDIATE => {}
            INSN_B_IMMEDIATE..=ENDINSN_B_IMMEDIATE => {}
            INSN_BL_IMMEDIATE..=ENDINSN_BL_IMMEDIATE => {}
            INSN_BR..=ENDINSN_BR => {}
            INSN_BRL..=ENDINSN_BRL => {}
            INSN_DIV..=ENDINSN_DIV => {}
            INSN_DIV_IMMEDIATE..=ENDINSN_DIV_IMMEDIATE => {}
            INSN_DIVF..=ENDINSN_DIVF => {}
            INSN_DIVS..=ENDINSN_DIVS => {}
            INSN_DIVS_IMMEDIATE..=ENDINSN_DIVS_IMMEDIATE => {}
            INSN_HALT..=ENDINSN_HALT => {}
            INSN_LDR..=ENDINSN_LDR => {}
            INSN_LDR_IMMEDIATE..=ENDINSN_LDR_IMMEDIATE => {}
            INSN_MOV..=ENDINSN_MOV => {}
            INSN_MOV_IMMEDIATE..=ENDINSN_MOV_IMMEDIATE => {}
            INSN_MOVS_IMMEDIATE..=ENDINSN_MOVS_IMMEDIATE => {}
            INSN_MUL..=ENDINSN_MUL => {}
            INSN_MUL_IMMEDIATE..=ENDINSN_MUL_IMMEDIATE => {}
            INSN_MULF..=ENDINSN_MULF => {}
            INSN_MULS..=ENDINSN_MULS => {}
            INSN_MULS_IMMEDIATE..=ENDINSN_MULS_IMMEDIATE => {}
            INSN_NOP..=ENDINSN_NOP => {}
            INSN_NOT..=ENDINSN_NOT => {}
            INSN_OR..=ENDINSN_OR => {}
            INSN_SHL..=ENDINSN_SHL => {}
            INSN_SHR..=ENDINSN_SHR => {}
            INSN_SHRS..=ENDINSN_SHRS => {}
            INSN_SUB..=ENDINSN_SUB => {}
            INSN_SUB_IMMEDIATE..=ENDINSN_SUB_IMMEDIATE => {}
            INSN_SUBF..=ENDINSN_SUBF => {}
            INSN_SUBS..=ENDINSN_SUBS => {}
            INSN_SUBS_IMMEDIATE..=ENDINSN_SUBS_IMMEDIATE => {}
            INSN_SYSCALL_IMMEDIATE..=ENDINSN_SYSCALL_IMMEDIATE => {}
            INSN_XOR..=ENDINSN_XOR => {}
            _ => {}
        }
    }

    #[inline(always)]
    pub fn add(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint + self.registers[b].uint },
        };
    }

    #[inline(always)]
    pub fn add_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<16>(insn, 10) as u64;
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint + imm },
        };
    }
}

#[inline(always)]
pub const fn reg(insn: u32, bit_pos: usize) -> usize {
    (insn as usize >> bit_pos) & 0x1F
}

#[inline(always)]
pub const fn immediate<const BITS: usize>(insn: u32, bit_pos: usize) -> usize {
    (insn as usize >> bit_pos) & ((1 << (BITS - 1)) - 1)
}
