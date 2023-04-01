# This file is automatically generated.
# It is not intended for manual editing.

# This module contains opcode constants for ISA version `1.4.0-pre`.

# `add Xdst Xlhs u17`
L0_ADD = 0b00000_00000000000000000_00000_00000
END_L0_ADD = 0b00000_11111111111111111_11111_11111
# `sub Xdst Xlhs u17`
L0_SUB = 0b00001_00000000000000000_00000_00000
END_L0_SUB = 0b00001_11111111111111111_11111_11111
# `mul Xdst Xlhs u17`
L0_MUL = 0b00010_00000000000000000_00000_00000
END_L0_MUL = 0b00010_11111111111111111_11111_11111
# `div Xdst Xlhs u17`
L0_DIV = 0b00011_00000000000000000_00000_00000
END_L0_DIV = 0b00011_11111111111111111_11111_11111
# `rem Xdst Xlhs u17`
L0_REM = 0b00100_00000000000000000_00000_00000
END_L0_REM = 0b00100_11111111111111111_11111_11111
# `divs Xdst Xlhs i17`
L0_DIVS = 0b00101_00000000000000000_00000_00000
END_L0_DIVS = 0b00101_11111111111111111_11111_11111
# `rems Xdst Xlhs i17`
L0_REMS = 0b00110_00000000000000000_00000_00000
END_L0_REMS = 0b00110_11111111111111111_11111_11111
# `ldr Xdst i22`
L0_LDR = 0b00111_0000000000000000000000_00000
END_L0_LDR = 0b00111_1111111111111111111111_11111
# `str i22 Xsrc`
L0_STR = 0b01000_00000_0000000000000000000000
END_L0_STR = 0b01000_11111_1111111111111111111111
# `mov Xdst u22`
L0_MOV = 0b01001_0000000000000000000000_00000
END_L0_MOV = 0b01001_1111111111111111111111_11111
# `movs Xdst i22`
L0_MOVS = 0b01010_0000000000000000000000_00000
END_L0_MOVS = 0b01010_1111111111111111111111_11111
# `branch i27`
L0_BRANCH = 0b01011_000000000000000000000000000
END_L0_BRANCH = 0b01011_111111111111111111111111111
# `branch.l i27`
L0_BRANCH_L = 0b01100_000000000000000000000000000
END_L0_BRANCH_L = 0b01100_111111111111111111111111111
# `branch.ld i27`
L0_BRANCH_LD = 0b01101_000000000000000000000000000
END_L0_BRANCH_LD = 0b01101_111111111111111111111111111
# `branch.l.ld i27`
L0_BRANCH_L_LD = 0b01110_000000000000000000000000000
END_L0_BRANCH_L_LD = 0b01110_111111111111111111111111111
# `branch.eq i22 Xcond`
L0_BRANCH_EQ = 0b01111_00000_0000000000000000000000
END_L0_BRANCH_EQ = 0b01111_11111_1111111111111111111111
# `branch.ne i22 Xcond`
L0_BRANCH_NE = 0b10000_00000_0000000000000000000000
END_L0_BRANCH_NE = 0b10000_11111_1111111111111111111111
# `branch.lt i22 Xcond`
L0_BRANCH_LT = 0b10001_00000_0000000000000000000000
END_L0_BRANCH_LT = 0b10001_11111_1111111111111111111111
# `branch.gt i22 Xcond`
L0_BRANCH_GT = 0b10010_00000_0000000000000000000000
END_L0_BRANCH_GT = 0b10010_11111_1111111111111111111111
# `branch.le i22 Xcond`
L0_BRANCH_LE = 0b10011_00000_0000000000000000000000
END_L0_BRANCH_LE = 0b10011_11111_1111111111111111111111
# `branch.ge i22 Xcond`
L0_BRANCH_GE = 0b10100_00000_0000000000000000000000
END_L0_BRANCH_GE = 0b10100_11111_1111111111111111111111
# `branch.zr i22 Xsrc`
L0_BRANCH_ZR = 0b10101_00000_0000000000000000000000
END_L0_BRANCH_ZR = 0b10101_11111_1111111111111111111111
# `branch.nz i22 Xsrc`
L0_BRANCH_NZ = 0b10110_00000_0000000000000000000000
END_L0_BRANCH_NZ = 0b10110_11111_1111111111111111111111
# `lea Xdst i22`
L0_LEA = 0b10111_0000000000000000000000_00000
END_L0_LEA = 0b10111_1111111111111111111111_11111
# `shl Xdst Xlhs u11`
L1_SHL = 0b11111_000000_00000000000_00000_00000
END_L1_SHL = 0b11111_000000_11111111111_11111_11111
# `shr Xdst Xlhs u11`
L1_SHR = 0b11111_000001_00000000000_00000_00000
END_L1_SHR = 0b11111_000001_11111111111_11111_11111
# `shrs Xdst Xlhs u11`
L1_SHRS = 0b11111_000010_00000000000_00000_00000
END_L1_SHRS = 0b11111_000010_11111111111_11111_11111
# `ldr Xdst Xsrc i11`
L1_LDR = 0b11111_000011_00000000000_00000_00000
END_L1_LDR = 0b11111_000011_11111111111_11111_11111
# `ldrb Xdst Xsrc i11`
L1_LDRB = 0b11111_000100_00000000000_00000_00000
END_L1_LDRB = 0b11111_000100_11111111111_11111_11111
# `ldrh Xdst Xsrc i11`
L1_LDRH = 0b11111_000101_00000000000_00000_00000
END_L1_LDRH = 0b11111_000101_11111111111_11111_11111
# `ldrw Xdst Xsrc i11`
L1_LDRW = 0b11111_000110_00000000000_00000_00000
END_L1_LDRW = 0b11111_000110_11111111111_11111_11111
# `str Xdst Xsrc i11`
L1_STR = 0b11111_000111_00000000000_00000_00000
END_L1_STR = 0b11111_000111_11111111111_11111_11111
# `strb Xdst Xsrc i11`
L1_STRB = 0b11111_001000_00000000000_00000_00000
END_L1_STRB = 0b11111_001000_11111111111_11111_11111
# `strh Xdst Xsrc i11`
L1_STRH = 0b11111_001001_00000000000_00000_00000
END_L1_STRH = 0b11111_001001_11111111111_11111_11111
# `strw Xdst Xsrc i11`
L1_STRW = 0b11111_001010_00000000000_00000_00000
END_L1_STRW = 0b11111_001010_11111111111_11111_11111
# `int u16`
L1_INT = 0b11111_001011_00000_0000000000000000
END_L1_INT = 0b11111_001011_00000_1111111111111111
# `ncall u21`
L1_NCALL = 0b11111_001100_000000000000000000000
END_L1_NCALL = 0b11111_001100_111111111111111111111
# `vcall u21`
L1_VCALL = 0b11111_001101_000000000000000000000
END_L1_VCALL = 0b11111_001101_111111111111111111111
# `add Xdst Xlhs Xrhs`
L2_ADD = 0b11111111111_000000_00000_00000_00000
END_L2_ADD = 0b11111111111_000000_11111_11111_11111
# `sub Xdst Xlhs Xrhs`
L2_SUB = 0b11111111111_000001_00000_00000_00000
END_L2_SUB = 0b11111111111_000001_11111_11111_11111
# `mul Xdst Xlhs Xrhs`
L2_MUL = 0b11111111111_000010_00000_00000_00000
END_L2_MUL = 0b11111111111_000010_11111_11111_11111
# `div Xdst Xlhs Xrhs`
L2_DIV = 0b11111111111_000011_00000_00000_00000
END_L2_DIV = 0b11111111111_000011_11111_11111_11111
# `rem Xdst Xlhs Xrhs`
L2_REM = 0b11111111111_000100_00000_00000_00000
END_L2_REM = 0b11111111111_000100_11111_11111_11111
# `divs Xdst Xlhs Xrhs`
L2_DIVS = 0b11111111111_000101_00000_00000_00000
END_L2_DIVS = 0b11111111111_000101_11111_11111_11111
# `rems Xdst Xlhs Xrhs`
L2_REMS = 0b11111111111_000110_00000_00000_00000
END_L2_REMS = 0b11111111111_000110_11111_11111_11111
# `addf Xdst Xlhs Xrhs`
L2_ADDF = 0b11111111111_000111_00000_00000_00000
END_L2_ADDF = 0b11111111111_000111_11111_11111_11111
# `subf Xdst Xlhs Xrhs`
L2_SUBF = 0b11111111111_001000_00000_00000_00000
END_L2_SUBF = 0b11111111111_001000_11111_11111_11111
# `mulf Xdst Xlhs Xrhs`
L2_MULF = 0b11111111111_001001_00000_00000_00000
END_L2_MULF = 0b11111111111_001001_11111_11111_11111
# `divf Xdst Xlhs Xrhs`
L2_DIVF = 0b11111111111_001010_00000_00000_00000
END_L2_DIVF = 0b11111111111_001010_11111_11111_11111
# `remf Xdst Xlhs Xrhs`
L2_REMF = 0b11111111111_001011_00000_00000_00000
END_L2_REMF = 0b11111111111_001011_11111_11111_11111
# `and Xdst Xlhs Xrhs`
L2_AND = 0b11111111111_001100_00000_00000_00000
END_L2_AND = 0b11111111111_001100_11111_11111_11111
# `or Xdst Xlhs Xrhs`
L2_OR = 0b11111111111_001101_00000_00000_00000
END_L2_OR = 0b11111111111_001101_11111_11111_11111
# `xor Xdst Xlhs Xrhs`
L2_XOR = 0b11111111111_001110_00000_00000_00000
END_L2_XOR = 0b11111111111_001110_11111_11111_11111
# `shl Xdst Xlhs Xrhs`
L2_SHL = 0b11111111111_001111_00000_00000_00000
END_L2_SHL = 0b11111111111_001111_11111_11111_11111
# `shr Xdst Xlhs Xrhs`
L2_SHR = 0b11111111111_010000_00000_00000_00000
END_L2_SHR = 0b11111111111_010000_11111_11111_11111
# `shrs Xdst Xlhs Xrhs`
L2_SHRS = 0b11111111111_010001_00000_00000_00000
END_L2_SHRS = 0b11111111111_010001_11111_11111_11111
# `cmp Xdst Xlhs Xrhs`
L2_CMP = 0b11111111111_010010_00000_00000_00000
END_L2_CMP = 0b11111111111_010010_11111_11111_11111
# `cmps Xdst Xlhs Xrhs`
L2_CMPS = 0b11111111111_010011_00000_00000_00000
END_L2_CMPS = 0b11111111111_010011_11111_11111_11111
# `cmpf Xdst Xlhs Xrhs`
L2_CMPF = 0b11111111111_010100_00000_00000_00000
END_L2_CMPF = 0b11111111111_010100_11111_11111_11111
# `not Xdst Xsrc`
L3_NOT = 0b11111111111111111_00000_00000_00000
END_L3_NOT = 0b11111111111111111_00000_11111_11111
# `mov Xdst Xsrc`
L3_MOV = 0b11111111111111111_00001_00000_00000
END_L3_MOV = 0b11111111111111111_00001_11111_11111
# `fti Xdst Xsrc`
L3_FTI = 0b11111111111111111_00010_00000_00000
END_L3_FTI = 0b11111111111111111_00010_11111_11111
# `itf Xdst Xsrc`
L3_ITF = 0b11111111111111111_00011_00000_00000
END_L3_ITF = 0b11111111111111111_00011_11111_11111
# `branch Xdst`
L4_BRANCH = 0b1111111111111111111111_00000_00000
END_L4_BRANCH = 0b1111111111111111111111_00000_11111
# `branch.l Xdst`
L4_BRANCH_L = 0b1111111111111111111111_00001_00000
END_L4_BRANCH_L = 0b1111111111111111111111_00001_11111
# `branch.ld Xsrc`
L4_BRANCH_LD = 0b1111111111111111111111_00010_00000
END_L4_BRANCH_LD = 0b1111111111111111111111_00010_11111
# `branch.l.ld Xsrc`
L4_BRANCH_L_LD = 0b1111111111111111111111_00011_00000
END_L4_BRANCH_L_LD = 0b1111111111111111111111_00011_11111
# `branch.bo Xdst`
L4_BRANCH_BO = 0b1111111111111111111111_00100_00000
END_L4_BRANCH_BO = 0b1111111111111111111111_00100_11111
# `branch.l.bo Xdst`
L4_BRANCH_L_BO = 0b1111111111111111111111_00101_00000
END_L4_BRANCH_L_BO = 0b1111111111111111111111_00101_11111
# `branch.ld.bo Xsrc`
L4_BRANCH_LD_BO = 0b1111111111111111111111_00110_00000
END_L4_BRANCH_LD_BO = 0b1111111111111111111111_00110_11111
# `branch.bo.ld Xsrc`
L4_BRANCH_BO_LD = 0b1111111111111111111111_00111_00000
END_L4_BRANCH_BO_LD = 0b1111111111111111111111_00111_11111
# `branch.bo.ld.bo Xsrc`
L4_BRANCH_BO_LD_BO = 0b1111111111111111111111_01000_00000
END_L4_BRANCH_BO_LD_BO = 0b1111111111111111111111_01000_11111
# `branch.l.ld.bo Xsrc`
L4_BRANCH_L_LD_BO = 0b1111111111111111111111_01001_00000
END_L4_BRANCH_L_LD_BO = 0b1111111111111111111111_01001_11111
# `branch.l.bo.ld Xsrc`
L4_BRANCH_L_BO_LD = 0b1111111111111111111111_01010_00000
END_L4_BRANCH_L_BO_LD = 0b1111111111111111111111_01010_11111
# `branch.l.bo.ld.bo Xsrc`
L4_BRANCH_L_BO_LD_BO = 0b1111111111111111111111_01011_00000
END_L4_BRANCH_L_BO_LD_BO = 0b1111111111111111111111_01011_11111
# `ncall Xid`
L4_NCALL = 0b1111111111111111111111_01100_00000
END_L4_NCALL = 0b1111111111111111111111_01100_11111
# `vcall Xid`
L4_VCALL = 0b1111111111111111111111_01101_00000
END_L4_VCALL = 0b1111111111111111111111_01101_11111
# `ldbo Xdst`
L4_LDBO = 0b1111111111111111111111_01110_00000
END_L4_LDBO = 0b1111111111111111111111_01110_11111
# `ldpc Xdst`
L4_LDPC = 0b1111111111111111111111_01111_00000
END_L4_LDPC = 0b1111111111111111111111_01111_11111
# `nop`
L5_NOP = 0b111111111111111111111111111_00000
END_L5_NOP = 0b111111111111111111111111111_00000
# `halt`
L5_HALT = 0b111111111111111111111111111_00001
END_L5_HALT = 0b111111111111111111111111111_00001
# `ret`
L5_RET = 0b111111111111111111111111111_00010
END_L5_RET = 0b111111111111111111111111111_00010
