pub mod noverify;
pub mod safe;

use std::cmp::Ordering;

use crate::asm::*;

#[derive(Clone, Copy)]
pub union Value {
    pub uint: u64,
    pub int: i64,
    pub float: f64,
    pub ord: Ordering,
}

pub trait Context: Sized {
    fn fetch_instruction(&mut self) -> u32;

    fn advance_counter(&mut self);

    fn counter(&mut self) -> *mut u32;

    fn set_counter(&mut self, counter: *mut u32);

    /// `add <X0>, <X1>, <X2>`
    fn add(&mut self, insn: u32);

    /// `add <X0>, <X1>, <i16>`
    fn add_imm(&mut self, insn: u32);

    /// `addf <X0>, <X1>, <X2>`
    fn addf(&mut self, insn: u32);

    /// `adds <X0>, <X1>, <X2>`
    fn adds(&mut self, insn: u32);

    /// `adds <X0>, <X1>, <i16>`
    fn adds_imm(&mut self, insn: u32);

    /// `and <X0>, <X1>, <X2>`
    fn and(&mut self, insn: u32);

    /// `b.eq <X0>, <i21>`
    fn b_eq_imm(&mut self, insn: u32);

    /// `b.ge <X0>, <i21>`
    fn b_ge_imm(&mut self, insn: u32);

    /// `b.gt <X0>, <i21>`
    fn b_gt_imm(&mut self, insn: u32);

    /// `b.le <X0>, <i21>`
    fn b_le_imm(&mut self, insn: u32);

    /// `b.lt <X0>, <i21>`
    fn b_lt_imm(&mut self, insn: u32);

    /// `b.nq <X0>, <i21>`
    fn b_ne_imm(&mut self, insn: u32);

    /// `b <i26>`
    fn b_imm(&mut self, insn: u32);

    /// `bl <i26>`
    fn bl_imm(&mut self, insn: u32);

    /// `br <X0>`
    fn br(&mut self, insn: u32);

    /// `brl <X0>`
    fn brl(&mut self, insn: u32);

    /// `div <X0>, <X1>, <X2>`
    fn div(&mut self, insn: u32);

    /// `div <X0>, <X1>, <i16>`
    fn div_imm(&mut self, insn: u32);

    /// `divf <X0>, <X1>, <X2>`
    fn divf(&mut self, insn: u32);

    /// `divs <X0>, <X1>, <X2>`
    fn divs(&mut self, insn: u32);

    /// `divs <X0>, <X1>, <i16>`
    fn divs_imm(&mut self, insn: u32);

    /// `halt`
    fn halt(&mut self, insn: u32);

    /// `interrupt <i26>`
    fn interrupt_imm(&mut self, insn: u32);

    /// `ldr <X0>, <X1>`
    fn ldr(&mut self, insn: u32);

    /// `ldr <X0>, <i21>`
    fn ldr_imm(&mut self, insn: u32);

    /// `mov <X0>, <X1>`
    fn mov(&mut self, insn: u32);

    /// `mov <X0>, <i21>`
    fn mov_imm(&mut self, insn: u32);

    /// `movs <X0>, <i21>`
    fn movs_imm(&mut self, insn: u32);

    /// `mul <X0>, <X1>, <X2>`
    fn mul(&mut self, insn: u32);

    /// `mul <X0>, <X1>, <i16>`
    fn mul_imm(&mut self, insn: u32);

    /// `mulf <X0>, <X1>, <X2>`
    fn mulf(&mut self, insn: u32);

    /// `muls <X0>, <X1>, <X2>`
    fn muls(&mut self, insn: u32);

    /// `muls <X0>, <X1>, <i16>`
    fn muls_imm(&mut self, insn: u32);

    /// `nop`
    fn nop(&mut self, insn: u32);

    /// `not <X0>, <X1>`
    fn not(&mut self, insn: u32);

    /// `or <X0>, <X1>, <X2>`
    fn or(&mut self, insn: u32);

    /// `shl <X0>, <X1>, <i16>`
    fn shl(&mut self, insn: u32);

    /// `shr <X0>, <X1>, <i16>`
    fn shr(&mut self, insn: u32);

    /// `shrs <X0>, <X1>, <i16>`
    fn shrs(&mut self, insn: u32);

    /// `str <X0>, <X1>`
    fn str(&mut self, insn: u32);

    /// `str <X0>, <i21>`
    fn str_imm(&mut self, insn: u32);

    /// `sub <X0>, <X1>, <X2>`
    fn sub(&mut self, insn: u32);

    /// `sub <X0>, <X1>, <i16>`
    fn sub_imm(&mut self, insn: u32);

    /// `subf <X0>, <X1>, <X2>`
    fn subf(&mut self, insn: u32);

    /// `subs <X0>, <X1>, <X2>`
    fn subs(&mut self, insn: u32);

    /// `subs <X0>, <X1>, <i16>`
    fn subs_imm(&mut self, insn: u32);

    /// `xor <X0>, <X1>, <X2>`
    fn xor(&mut self, insn: u32);

    fn decode_instruction(&mut self) {
        let insn = self.fetch_instruction();
        match insn {
            INSN_ADD..=ENDINSN_ADD => self.add(insn),
            INSN_ADD_IMMEDIATE..=ENDINSN_ADD_IMMEDIATE => self.add_imm(insn),
            INSN_ADDF..=ENDINSN_ADDF => self.addf(insn),
            INSN_ADDS..=ENDINSN_ADDS => self.adds(insn),
            INSN_ADDS_IMMEDIATE..=ENDINSN_ADDS_IMMEDIATE => self.adds_imm(insn),
            INSN_AND..=ENDINSN_AND => self.and(insn),
            INSN_B_EQ_IMMEDIATE..=ENDINSN_B_EQ_IMMEDIATE => self.b_eq_imm(insn),
            INSN_B_GE_IMMEDIATE..=ENDINSN_B_GE_IMMEDIATE => self.b_ge_imm(insn),
            INSN_B_GT_IMMEDIATE..=ENDINSN_B_GT_IMMEDIATE => self.b_gt_imm(insn),
            INSN_B_LE_IMMEDIATE..=ENDINSN_B_LE_IMMEDIATE => self.b_le_imm(insn),
            INSN_B_LT_IMMEDIATE..=ENDINSN_B_LT_IMMEDIATE => self.b_lt_imm(insn),
            INSN_B_NE_IMMEDIATE..=ENDINSN_B_NE_IMMEDIATE => self.b_ne_imm(insn),
            INSN_B_IMMEDIATE..=ENDINSN_B_IMMEDIATE => self.b_imm(insn),
            INSN_BL_IMMEDIATE..=ENDINSN_BL_IMMEDIATE => self.bl_imm(insn),
            INSN_BR..=ENDINSN_BR => self.br(insn),
            INSN_BRL..=ENDINSN_BRL => self.brl(insn),
            INSN_DIV..=ENDINSN_DIV => self.div(insn),
            INSN_DIV_IMMEDIATE..=ENDINSN_DIV_IMMEDIATE => self.div_imm(insn),
            INSN_DIVF..=ENDINSN_DIVF => self.divf(insn),
            INSN_DIVS..=ENDINSN_DIVS => self.divs(insn),
            INSN_DIVS_IMMEDIATE..=ENDINSN_DIVS_IMMEDIATE => self.divs_imm(insn),
            INSN_HALT..=ENDINSN_HALT => self.halt(insn),
            INSN_INTERRUPT_IMMEDIATE..=ENDINSN_INTERRUPT_IMMEDIATE => self.interrupt_imm(insn),
            INSN_LDR..=ENDINSN_LDR => self.ldr(insn),
            INSN_LDR_IMMEDIATE..=ENDINSN_LDR_IMMEDIATE => self.ldr_imm(insn),
            INSN_MOV..=ENDINSN_MOV => self.mov(insn),
            INSN_MOV_IMMEDIATE..=ENDINSN_MOV_IMMEDIATE => self.mov_imm(insn),
            INSN_MOVS_IMMEDIATE..=ENDINSN_MOVS_IMMEDIATE => self.movs_imm(insn),
            INSN_MUL..=ENDINSN_MUL => self.mul(insn),
            INSN_MUL_IMMEDIATE..=ENDINSN_MUL_IMMEDIATE => self.mul_imm(insn),
            INSN_MULF..=ENDINSN_MULF => self.mulf(insn),
            INSN_MULS..=ENDINSN_MULS => self.muls(insn),
            INSN_MULS_IMMEDIATE..=ENDINSN_MULS_IMMEDIATE => self.muls_imm(insn),
            INSN_NOP..=ENDINSN_NOP => self.nop(insn),
            INSN_NOT..=ENDINSN_NOT => self.not(insn),
            INSN_OR..=ENDINSN_OR => self.or(insn),
            INSN_SHL..=ENDINSN_SHL => self.shl(insn),
            INSN_SHR..=ENDINSN_SHR => self.shr(insn),
            INSN_SHRS..=ENDINSN_SHRS => self.shrs(insn),
            INSN_STR..=ENDINSN_STR => self.str(insn),
            INSN_STR_IMMEDIATE..=ENDINSN_STR_IMMEDIATE => self.str_imm(insn),
            INSN_SUB..=ENDINSN_SUB => self.sub(insn),
            INSN_SUB_IMMEDIATE..=ENDINSN_SUB_IMMEDIATE => self.sub_imm(insn),
            INSN_SUBF..=ENDINSN_SUBF => self.subf(insn),
            INSN_SUBS..=ENDINSN_SUBS => self.subs(insn),
            INSN_SUBS_IMMEDIATE..=ENDINSN_SUBS_IMMEDIATE => self.subs_imm(insn),
            INSN_XOR..=ENDINSN_XOR => self.xor(insn),
            _ => {}
        }
    }
}

#[inline(always)]
pub const fn reg(insn: u32, bit_pos: usize) -> usize {
    (insn as usize >> bit_pos) & 0x1F
}

#[inline(always)]
pub const fn immediate<const BITS: usize>(insn: u32, bit_pos: usize) -> u64 {
    (insn as u64 >> bit_pos) & ((1 << BITS) - 1)
}

#[inline(always)]
pub const fn signed_immediate<const BITS: usize>(insn: u32, bit_pos: usize) -> i64 {
    (((((insn >> bit_pos) & ((1 << BITS) - 1)) << (32 - BITS)) as i32) >> (32 - BITS)) as i64
}
