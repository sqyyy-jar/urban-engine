#![allow(clippy::unusual_byte_groupings)]

//! This module contains constants for all instructions
//!
//! ## Instruction
//!
//! 0bXXXXX_000000000000000000000000000
//!
//! `X` represents the id of the instruction

// ==========[ Generated - do not edit ]==========
/// `add <X0>, <X1>, <X2>`
pub const INSN_ADD: u32 = 0b000000_00000000000_00000_00000_00000;
pub const ENDINSN_ADD: u32 = 0b000000_11111111111_11111_11111_11111;
/// `add <X0>, <X1>, <i16>`
pub const INSN_ADD_IMMEDIATE: u32 = 0b000001_0000000000000000_00000_00000;
pub const ENDINSN_ADD_IMMEDIATE: u32 = 0b000001_1111111111111111_11111_11111;
/// `addf <X0>, <X1>, <X2>`
pub const INSN_ADDF: u32 = 0b000010_00000000000_00000_00000_00000;
pub const ENDINSN_ADDF: u32 = 0b000010_11111111111_11111_11111_11111;
/// `adds <X0>, <X1>, <X2>`
pub const INSN_ADDS: u32 = 0b000011_00000000000_00000_00000_00000;
pub const ENDINSN_ADDS: u32 = 0b000011_11111111111_11111_11111_11111;
/// `adds <X0>, <X1>, <i16>`
pub const INSN_ADDS_IMMEDIATE: u32 = 0b000100_0000000000000000_00000_00000;
pub const ENDINSN_ADDS_IMMEDIATE: u32 = 0b000100_1111111111111111_11111_11111;
/// `and <X0>, <X1>, <X2>`
pub const INSN_AND: u32 = 0b000101_00000000000_00000_00000_00000;
pub const ENDINSN_AND: u32 = 0b000101_11111111111_11111_11111_11111;
/// `b.eq <X0>, <i21>`
pub const INSN_B_EQ_IMMEDIATE: u32 = 0b000110_000000000000000000000_00000;
pub const ENDINSN_B_EQ_IMMEDIATE: u32 = 0b000110_111111111111111111111_11111;
/// `b.ge <X0>, <i21>`
pub const INSN_B_GE_IMMEDIATE: u32 = 0b000111_000000000000000000000_00000;
pub const ENDINSN_B_GE_IMMEDIATE: u32 = 0b000111_111111111111111111111_11111;
/// `b.gt <X0>, <i21>`
pub const INSN_B_GT_IMMEDIATE: u32 = 0b001000_000000000000000000000_00000;
pub const ENDINSN_B_GT_IMMEDIATE: u32 = 0b001000_111111111111111111111_11111;
/// `b.le <X0>, <i21>`
pub const INSN_B_LE_IMMEDIATE: u32 = 0b001001_000000000000000000000_00000;
pub const ENDINSN_B_LE_IMMEDIATE: u32 = 0b001001_111111111111111111111_11111;
/// `b.lt <X0>, <i21>`
pub const INSN_B_LT_IMMEDIATE: u32 = 0b001010_000000000000000000000_00000;
pub const ENDINSN_B_LT_IMMEDIATE: u32 = 0b001010_111111111111111111111_11111;
/// `b.nq <X0>, <i21>`
pub const INSN_B_NQ_IMMEDIATE: u32 = 0b001011_000000000000000000000_00000;
pub const ENDINSN_B_NQ_IMMEDIATE: u32 = 0b001011_111111111111111111111_11111;
/// `b <i26>`
pub const INSN_B_IMMEDIATE: u32 = 0b001100_00000000000000000000000000;
pub const ENDINSN_B_IMMEDIATE: u32 = 0b001100_11111111111111111111111111;
/// `bl <i26>`
pub const INSN_BL_IMMEDIATE: u32 = 0b001101_00000000000000000000000000;
pub const ENDINSN_BL_IMMEDIATE: u32 = 0b001101_11111111111111111111111111;
/// `br <X0>`
pub const INSN_BR: u32 = 0b001110_000000000000000000000_00000;
pub const ENDINSN_BR: u32 = 0b001110_111111111111111111111_11111;
/// `brl <X0>`
pub const INSN_BRL: u32 = 0b001111_000000000000000000000_00000;
pub const ENDINSN_BRL: u32 = 0b001111_111111111111111111111_11111;
/// `div <X0>, <X1>, <X2>`
pub const INSN_DIV: u32 = 0b010000_00000000000_00000_00000_00000;
pub const ENDINSN_DIV: u32 = 0b010000_11111111111_11111_11111_11111;
/// `div <X0>, <X1>, <i16>`
pub const INSN_DIV_IMMEDIATE: u32 = 0b010001_0000000000000000_00000_00000;
pub const ENDINSN_DIV_IMMEDIATE: u32 = 0b010001_1111111111111111_11111_11111;
/// `divf <X0>, <X1>, <X2>`
pub const INSN_DIVF: u32 = 0b010010_00000000000_00000_00000_00000;
pub const ENDINSN_DIVF: u32 = 0b010010_11111111111_11111_11111_11111;
/// `divs <X0>, <X1>, <X2>`
pub const INSN_DIVS: u32 = 0b010011_00000000000_00000_00000_00000;
pub const ENDINSN_DIVS: u32 = 0b010011_11111111111_11111_11111_11111;
/// `divs <X0>, <X1>, <i16>`
pub const INSN_DIVS_IMMEDIATE: u32 = 0b010100_0000000000000000_00000_00000;
pub const ENDINSN_DIVS_IMMEDIATE: u32 = 0b010100_1111111111111111_11111_11111;
/// `halt`
pub const INSN_HALT: u32 = 0b010101_00000000000000000000000000;
pub const ENDINSN_HALT: u32 = 0b010101_11111111111111111111111111;
/// `interrupt <i26>`
pub const INSN_INTERRUPT_IMMEDIATE: u32 = 0b010110_00000000000000000000000000;
pub const ENDINSN_INTERRUPT_IMMEDIATE: u32 = 0b010110_11111111111111111111111111;
/// `ldr <X0>, <X1>`
pub const INSN_LDR: u32 = 0b010111_0000000000000000_00000_00000;
pub const ENDINSN_LDR: u32 = 0b010111_1111111111111111_11111_11111;
/// `ldr <X0>, <i21>`
pub const INSN_LDR_IMMEDIATE: u32 = 0b011000_000000000000000000000_00000;
pub const ENDINSN_LDR_IMMEDIATE: u32 = 0b011000_111111111111111111111_11111;
/// `mov <X0>, <X1>`
pub const INSN_MOV: u32 = 0b011001_0000000000000000_00000_00000;
pub const ENDINSN_MOV: u32 = 0b011001_1111111111111111_11111_11111;
/// `mov <X0>, <i21>`
pub const INSN_MOV_IMMEDIATE: u32 = 0b011010_000000000000000000000_00000;
pub const ENDINSN_MOV_IMMEDIATE: u32 = 0b011010_111111111111111111111_11111;
/// `movs <X0>, <i21>`
pub const INSN_MOVS_IMMEDIATE: u32 = 0b011011_000000000000000000000_00000;
pub const ENDINSN_MOVS_IMMEDIATE: u32 = 0b011011_111111111111111111111_11111;
/// `mul <X0>, <X1>, <X2>`
pub const INSN_MUL: u32 = 0b011100_00000000000_00000_00000_00000;
pub const ENDINSN_MUL: u32 = 0b011100_11111111111_11111_11111_11111;
/// `mul <X0>, <X1>, <i16>`
pub const INSN_MUL_IMMEDIATE: u32 = 0b011101_0000000000000000_00000_00000;
pub const ENDINSN_MUL_IMMEDIATE: u32 = 0b011101_1111111111111111_11111_11111;
/// `mulf <X0>, <X1>, <X2>`
pub const INSN_MULF: u32 = 0b011110_00000000000_00000_00000_00000;
pub const ENDINSN_MULF: u32 = 0b011110_11111111111_11111_11111_11111;
/// `muls <X0>, <X1>, <X2>`
pub const INSN_MULS: u32 = 0b011111_00000000000_00000_00000_00000;
pub const ENDINSN_MULS: u32 = 0b011111_11111111111_11111_11111_11111;
/// `muls <X0>, <X1>, <i16>`
pub const INSN_MULS_IMMEDIATE: u32 = 0b100000_0000000000000000_00000_00000;
pub const ENDINSN_MULS_IMMEDIATE: u32 = 0b100000_1111111111111111_11111_11111;
/// `nop`
pub const INSN_NOP: u32 = 0b100001_00000000000000000000000000;
pub const ENDINSN_NOP: u32 = 0b100001_11111111111111111111111111;
/// `not <X0>, <X1>`
pub const INSN_NOT: u32 = 0b100010_0000000000000000_00000_00000;
pub const ENDINSN_NOT: u32 = 0b100010_1111111111111111_11111_11111;
/// `or <X0>, <X1>, <X2>`
pub const INSN_OR: u32 = 0b100011_00000000000_00000_00000_00000;
pub const ENDINSN_OR: u32 = 0b100011_11111111111_11111_11111_11111;
/// `shl <X0>, <X1>, <i16>`
pub const INSN_SHL: u32 = 0b100100_0000000000000000_00000_00000;
pub const ENDINSN_SHL: u32 = 0b100100_1111111111111111_11111_11111;
/// `shr <X0>, <X1>, <i16>`
pub const INSN_SHR: u32 = 0b100101_0000000000000000_00000_00000;
pub const ENDINSN_SHR: u32 = 0b100101_1111111111111111_11111_11111;
/// `shrs <X0>, <X1>, <i16>`
pub const INSN_SHRS: u32 = 0b100110_0000000000000000_00000_00000;
pub const ENDINSN_SHRS: u32 = 0b100110_1111111111111111_11111_11111;
/// `str <X0>, <X1>`
pub const INSN_STR: u32 = 0b100111_0000000000000000_00000_00000;
pub const ENDINSN_STR: u32 = 0b100111_1111111111111111_11111_11111;
/// `str <X0>, <i21>`
pub const INSN_STR_IMMEDIATE: u32 = 0b101000_000000000000000000000_00000;
pub const ENDINSN_STR_IMMEDIATE: u32 = 0b101000_111111111111111111111_11111;
/// `sub <X0>, <X1>, <X2>`
pub const INSN_SUB: u32 = 0b101001_00000000000_00000_00000_00000;
pub const ENDINSN_SUB: u32 = 0b101001_11111111111_11111_11111_11111;
/// `sub <X0>, <X1>, <i16>`
pub const INSN_SUB_IMMEDIATE: u32 = 0b101010_0000000000000000_00000_00000;
pub const ENDINSN_SUB_IMMEDIATE: u32 = 0b101010_1111111111111111_11111_11111;
/// `subf <X0>, <X1>, <X2>`
pub const INSN_SUBF: u32 = 0b101011_00000000000_00000_00000_00000;
pub const ENDINSN_SUBF: u32 = 0b101011_11111111111_11111_11111_11111;
/// `subs <X0>, <X1>, <X2>`
pub const INSN_SUBS: u32 = 0b101100_00000000000_00000_00000_00000;
pub const ENDINSN_SUBS: u32 = 0b101100_11111111111_11111_11111_11111;
/// `subs <X0>, <X1>, <i16>`
pub const INSN_SUBS_IMMEDIATE: u32 = 0b101101_0000000000000000_00000_00000;
pub const ENDINSN_SUBS_IMMEDIATE: u32 = 0b101101_1111111111111111_11111_11111;
/// `xor <X0>, <X1>, <X2>`
pub const INSN_XOR: u32 = 0b101110_00000000000_00000_00000_00000;
pub const ENDINSN_XOR: u32 = 0b101110_11111111111_11111_11111_11111;
