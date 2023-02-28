// This file is automatically generated.
// It is not intended for manual editing.

//! This module contains opcode constants for ISA version `1.0`.

/// `add Xdst Xlhs u17`
pub const L0_ADD: u32 = 0b00000_00000_00000_00000000000000000;
pub const END_L0_ADD: u32 = 0b00000_11111_11111_11111111111111111;
/// `sub Xdst Xlhs u17`
pub const L0_SUB: u32 = 0b00001_00000_00000_00000000000000000;
pub const END_L0_SUB: u32 = 0b00001_11111_11111_11111111111111111;
/// `mul Xdst Xlhs u17`
pub const L0_MUL: u32 = 0b00010_00000_00000_00000000000000000;
pub const END_L0_MUL: u32 = 0b00010_11111_11111_11111111111111111;
/// `div Xdst Xlhs u17`
pub const L0_DIV: u32 = 0b00011_00000_00000_00000000000000000;
pub const END_L0_DIV: u32 = 0b00011_11111_11111_11111111111111111;
/// `adds Xdst Xlhs i17`
pub const L0_ADDS: u32 = 0b00100_00000_00000_00000000000000000;
pub const END_L0_ADDS: u32 = 0b00100_11111_11111_11111111111111111;
/// `subs Xdst Xlhs i17`
pub const L0_SUBS: u32 = 0b00101_00000_00000_00000000000000000;
pub const END_L0_SUBS: u32 = 0b00101_11111_11111_11111111111111111;
/// `muls Xdst Xlhs i17`
pub const L0_MULS: u32 = 0b00110_00000_00000_00000000000000000;
pub const END_L0_MULS: u32 = 0b00110_11111_11111_11111111111111111;
/// `divs Xdst Xlhs i17`
pub const L0_DIVS: u32 = 0b00111_00000_00000_00000000000000000;
pub const END_L0_DIVS: u32 = 0b00111_11111_11111_11111111111111111;
/// `ldr Xdst i22`
pub const L0_LDR: u32 = 0b01000_00000_0000000000000000000000;
pub const END_L0_LDR: u32 = 0b01000_11111_1111111111111111111111;
/// `str i22 Xsrc`
pub const L0_STR: u32 = 0b01001_0000000000000000000000_00000;
pub const END_L0_STR: u32 = 0b01001_1111111111111111111111_11111;
/// `mov Xdst u22`
pub const L0_MOV: u32 = 0b01010_00000_0000000000000000000000;
pub const END_L0_MOV: u32 = 0b01010_11111_1111111111111111111111;
/// `movs Xdst i22`
pub const L0_MOVS: u32 = 0b01011_00000_0000000000000000000000;
pub const END_L0_MOVS: u32 = 0b01011_11111_1111111111111111111111;
/// `branch i27`
pub const L0_BRANCH: u32 = 0b01100_000000000000000000000000000;
pub const END_L0_BRANCH: u32 = 0b01100_111111111111111111111111111;
/// `branch.l i27`
pub const L0_BRANCH_L: u32 = 0b01101_000000000000000000000000000;
pub const END_L0_BRANCH_L: u32 = 0b01101_111111111111111111111111111;
/// `branch.ld i27`
pub const L0_BRANCH_LD: u32 = 0b01110_000000000000000000000000000;
pub const END_L0_BRANCH_LD: u32 = 0b01110_111111111111111111111111111;
/// `branch.l.ld i27`
pub const L0_BRANCH_L_LD: u32 = 0b01111_000000000000000000000000000;
pub const END_L0_BRANCH_L_LD: u32 = 0b01111_111111111111111111111111111;
/// `branch.eq i22 Xcond`
pub const L0_BRANCH_EQ: u32 = 0b10000_0000000000000000000000_00000;
pub const END_L0_BRANCH_EQ: u32 = 0b10000_1111111111111111111111_11111;
/// `branch.ne i22 Xcond`
pub const L0_BRANCH_NE: u32 = 0b10001_0000000000000000000000_00000;
pub const END_L0_BRANCH_NE: u32 = 0b10001_1111111111111111111111_11111;
/// `branch.lt i22 Xcond`
pub const L0_BRANCH_LT: u32 = 0b10010_0000000000000000000000_00000;
pub const END_L0_BRANCH_LT: u32 = 0b10010_1111111111111111111111_11111;
/// `branch.gt i22 Xcond`
pub const L0_BRANCH_GT: u32 = 0b10011_0000000000000000000000_00000;
pub const END_L0_BRANCH_GT: u32 = 0b10011_1111111111111111111111_11111;
/// `branch.le i22 Xcond`
pub const L0_BRANCH_LE: u32 = 0b10100_0000000000000000000000_00000;
pub const END_L0_BRANCH_LE: u32 = 0b10100_1111111111111111111111_11111;
/// `branch.ge i22 Xcond`
pub const L0_BRANCH_GE: u32 = 0b10101_0000000000000000000000_00000;
pub const END_L0_BRANCH_GE: u32 = 0b10101_1111111111111111111111_11111;
/// `shl Xdst Xlhs u11`
pub const L1_SHL: u32 = 0b11111_000000_00000_00000_00000000000;
pub const END_L1_SHL: u32 = 0b11111_000000_11111_11111_11111111111;
/// `shr Xdst Xlhs u11`
pub const L1_SHR: u32 = 0b11111_000001_00000_00000_00000000000;
pub const END_L1_SHR: u32 = 0b11111_000001_11111_11111_11111111111;
/// `shrs Xdst Xlhs u11`
pub const L1_SHRS: u32 = 0b11111_000010_00000_00000_00000000000;
pub const END_L1_SHRS: u32 = 0b11111_000010_11111_11111_11111111111;
/// `ldr Xdst Xsrc i11`
pub const L1_LDR: u32 = 0b11111_000011_00000_00000_00000000000;
pub const END_L1_LDR: u32 = 0b11111_000011_11111_11111_11111111111;
/// `ldrb Xdst Xsrc i11`
pub const L1_LDRB: u32 = 0b11111_000100_00000_00000_00000000000;
pub const END_L1_LDRB: u32 = 0b11111_000100_11111_11111_11111111111;
/// `ldrh Xdst Xsrc i11`
pub const L1_LDRH: u32 = 0b11111_000101_00000_00000_00000000000;
pub const END_L1_LDRH: u32 = 0b11111_000101_11111_11111_11111111111;
/// `ldrw Xdst Xsrc i11`
pub const L1_LDRW: u32 = 0b11111_000110_00000_00000_00000000000;
pub const END_L1_LDRW: u32 = 0b11111_000110_11111_11111_11111111111;
/// `str Xdst Xsrc i11`
pub const L1_STR: u32 = 0b11111_000111_00000_00000_00000000000;
pub const END_L1_STR: u32 = 0b11111_000111_11111_11111_11111111111;
/// `strb Xdst Xsrc i11`
pub const L1_STRB: u32 = 0b11111_001000_00000_00000_00000000000;
pub const END_L1_STRB: u32 = 0b11111_001000_11111_11111_11111111111;
/// `strh Xdst Xsrc i11`
pub const L1_STRH: u32 = 0b11111_001001_00000_00000_00000000000;
pub const END_L1_STRH: u32 = 0b11111_001001_11111_11111_11111111111;
/// `strw Xdst Xsrc i11`
pub const L1_STRW: u32 = 0b11111_001010_00000_00000_00000000000;
pub const END_L1_STRW: u32 = 0b11111_001010_11111_11111_11111111111;
/// `int u16`
pub const L1_INT: u32 = 0b11111_001011_00000_0000000000000000;
pub const END_L1_INT: u32 = 0b11111_001011_00000_1111111111111111;
/// `n_call u21`
pub const L1_N_CALL: u32 = 0b11111_001100_000000000000000000000;
pub const END_L1_N_CALL: u32 = 0b11111_001100_111111111111111111111;
/// `v_call u21`
pub const L1_V_CALL: u32 = 0b11111_001101_000000000000000000000;
pub const END_L1_V_CALL: u32 = 0b11111_001101_111111111111111111111;
/// `add Xdst Xlhs Xrhs`
pub const L2_ADD: u32 = 0b11111111111_000000_00000_00000_00000;
pub const END_L2_ADD: u32 = 0b11111111111_000000_11111_11111_11111;
/// `sub Xdst Xlhs Xrhs`
pub const L2_SUB: u32 = 0b11111111111_000001_00000_00000_00000;
pub const END_L2_SUB: u32 = 0b11111111111_000001_11111_11111_11111;
/// `mul Xdst Xlhs Xrhs`
pub const L2_MUL: u32 = 0b11111111111_000010_00000_00000_00000;
pub const END_L2_MUL: u32 = 0b11111111111_000010_11111_11111_11111;
/// `div Xdst Xlhs Xrhs`
pub const L2_DIV: u32 = 0b11111111111_000011_00000_00000_00000;
pub const END_L2_DIV: u32 = 0b11111111111_000011_11111_11111_11111;
/// `adds Xdst Xlhs Xrhs`
pub const L2_ADDS: u32 = 0b11111111111_000100_00000_00000_00000;
pub const END_L2_ADDS: u32 = 0b11111111111_000100_11111_11111_11111;
/// `subs Xdst Xlhs Xrhs`
pub const L2_SUBS: u32 = 0b11111111111_000101_00000_00000_00000;
pub const END_L2_SUBS: u32 = 0b11111111111_000101_11111_11111_11111;
/// `muls Xdst Xlhs Xrhs`
pub const L2_MULS: u32 = 0b11111111111_000110_00000_00000_00000;
pub const END_L2_MULS: u32 = 0b11111111111_000110_11111_11111_11111;
/// `divs Xdst Xlhs Xrhs`
pub const L2_DIVS: u32 = 0b11111111111_000111_00000_00000_00000;
pub const END_L2_DIVS: u32 = 0b11111111111_000111_11111_11111_11111;
/// `addf Xdst Xlhs Xrhs`
pub const L2_ADDF: u32 = 0b11111111111_001000_00000_00000_00000;
pub const END_L2_ADDF: u32 = 0b11111111111_001000_11111_11111_11111;
/// `subf Xdst Xlhs Xrhs`
pub const L2_SUBF: u32 = 0b11111111111_001001_00000_00000_00000;
pub const END_L2_SUBF: u32 = 0b11111111111_001001_11111_11111_11111;
/// `mulf Xdst Xlhs Xrhs`
pub const L2_MULF: u32 = 0b11111111111_001010_00000_00000_00000;
pub const END_L2_MULF: u32 = 0b11111111111_001010_11111_11111_11111;
/// `divf Xdst Xlhs Xrhs`
pub const L2_DIVF: u32 = 0b11111111111_001011_00000_00000_00000;
pub const END_L2_DIVF: u32 = 0b11111111111_001011_11111_11111_11111;
/// `and Xdst Xlhs Xrhs`
pub const L2_AND: u32 = 0b11111111111_001100_00000_00000_00000;
pub const END_L2_AND: u32 = 0b11111111111_001100_11111_11111_11111;
/// `or Xdst Xlhs Xrhs`
pub const L2_OR: u32 = 0b11111111111_001101_00000_00000_00000;
pub const END_L2_OR: u32 = 0b11111111111_001101_11111_11111_11111;
/// `xor Xdst Xlhs Xrhs`
pub const L2_XOR: u32 = 0b11111111111_001110_00000_00000_00000;
pub const END_L2_XOR: u32 = 0b11111111111_001110_11111_11111_11111;
/// `shl Xdst Xlhs Xrhs`
pub const L2_SHL: u32 = 0b11111111111_001111_00000_00000_00000;
pub const END_L2_SHL: u32 = 0b11111111111_001111_11111_11111_11111;
/// `shr Xdst Xlhs Xrhs`
pub const L2_SHR: u32 = 0b11111111111_010000_00000_00000_00000;
pub const END_L2_SHR: u32 = 0b11111111111_010000_11111_11111_11111;
/// `shrs Xdst Xlhs Xrhs`
pub const L2_SHRS: u32 = 0b11111111111_010001_00000_00000_00000;
pub const END_L2_SHRS: u32 = 0b11111111111_010001_11111_11111_11111;
/// `cmp Xdst Xlhs Xrhs`
pub const L2_CMP: u32 = 0b11111111111_010010_00000_00000_00000;
pub const END_L2_CMP: u32 = 0b11111111111_010010_11111_11111_11111;
/// `cmps Xdst Xlhs Xrhs`
pub const L2_CMPS: u32 = 0b11111111111_010011_00000_00000_00000;
pub const END_L2_CMPS: u32 = 0b11111111111_010011_11111_11111_11111;
/// `cmpf Xdst Xlhs Xrhs`
pub const L2_CMPF: u32 = 0b11111111111_010100_00000_00000_00000;
pub const END_L2_CMPF: u32 = 0b11111111111_010100_11111_11111_11111;
/// `not Xdst Xsrc`
pub const L3_NOT: u32 = 0b11111111111111111_00000_00000_00000;
pub const END_L3_NOT: u32 = 0b11111111111111111_00000_11111_11111;
/// `mov Xdst Xsrc`
pub const L3_MOV: u32 = 0b11111111111111111_00001_00000_00000;
pub const END_L3_MOV: u32 = 0b11111111111111111_00001_11111_11111;
/// `branch Xdst`
pub const L4_BRANCH: u32 = 0b1111111111111111111111_00000_00000;
pub const END_L4_BRANCH: u32 = 0b1111111111111111111111_00000_11111;
/// `branch.l Xdst`
pub const L4_BRANCH_L: u32 = 0b1111111111111111111111_00001_00000;
pub const END_L4_BRANCH_L: u32 = 0b1111111111111111111111_00001_11111;
/// `branch.ld Xdst`
pub const L4_BRANCH_LD: u32 = 0b1111111111111111111111_00010_00000;
pub const END_L4_BRANCH_LD: u32 = 0b1111111111111111111111_00010_11111;
/// `branch.l.ld Xdst`
pub const L4_BRANCH_L_LD: u32 = 0b1111111111111111111111_00011_00000;
pub const END_L4_BRANCH_L_LD: u32 = 0b1111111111111111111111_00011_11111;
/// `nop`
pub const L5_NOP: u32 = 0b111111111111111111111111111_00000;
pub const END_L5_NOP: u32 = 0b111111111111111111111111111_00000;
/// `halt`
pub const L5_HALT: u32 = 0b111111111111111111111111111_00001;
pub const END_L5_HALT: u32 = 0b111111111111111111111111111_00001;
