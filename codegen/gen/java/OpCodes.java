// This file is automatically generated.
// It is not intended for manual editing.

/**
 * This class contains opcode constants for ISA version {@code 1.4.0-pre}.
 */
public class OpCodes {
    /**
     * {@code add Xdst Xlhs u17}
     */
    public static final int L0_ADD = 0b00000_00000000000000000_00000_00000;
    public static final int END_L0_ADD = 0b00000_11111111111111111_11111_11111;
    /**
     * {@code sub Xdst Xlhs u17}
     */
    public static final int L0_SUB = 0b00001_00000000000000000_00000_00000;
    public static final int END_L0_SUB = 0b00001_11111111111111111_11111_11111;
    /**
     * {@code mul Xdst Xlhs u17}
     */
    public static final int L0_MUL = 0b00010_00000000000000000_00000_00000;
    public static final int END_L0_MUL = 0b00010_11111111111111111_11111_11111;
    /**
     * {@code div Xdst Xlhs u17}
     */
    public static final int L0_DIV = 0b00011_00000000000000000_00000_00000;
    public static final int END_L0_DIV = 0b00011_11111111111111111_11111_11111;
    /**
     * {@code rem Xdst Xlhs u17}
     */
    public static final int L0_REM = 0b00100_00000000000000000_00000_00000;
    public static final int END_L0_REM = 0b00100_11111111111111111_11111_11111;
    /**
     * {@code divs Xdst Xlhs i17}
     */
    public static final int L0_DIVS = 0b00101_00000000000000000_00000_00000;
    public static final int END_L0_DIVS = 0b00101_11111111111111111_11111_11111;
    /**
     * {@code rems Xdst Xlhs i17}
     */
    public static final int L0_REMS = 0b00110_00000000000000000_00000_00000;
    public static final int END_L0_REMS = 0b00110_11111111111111111_11111_11111;
    /**
     * {@code ldr Xdst i22}
     */
    public static final int L0_LDR = 0b00111_0000000000000000000000_00000;
    public static final int END_L0_LDR = 0b00111_1111111111111111111111_11111;
    /**
     * {@code str i22 Xsrc}
     */
    public static final int L0_STR = 0b01000_00000_0000000000000000000000;
    public static final int END_L0_STR = 0b01000_11111_1111111111111111111111;
    /**
     * {@code mov Xdst u22}
     */
    public static final int L0_MOV = 0b01001_0000000000000000000000_00000;
    public static final int END_L0_MOV = 0b01001_1111111111111111111111_11111;
    /**
     * {@code movs Xdst i22}
     */
    public static final int L0_MOVS = 0b01010_0000000000000000000000_00000;
    public static final int END_L0_MOVS = 0b01010_1111111111111111111111_11111;
    /**
     * {@code branch i27}
     */
    public static final int L0_BRANCH = 0b01011_000000000000000000000000000;
    public static final int END_L0_BRANCH = 0b01011_111111111111111111111111111;
    /**
     * {@code branch.l i27}
     */
    public static final int L0_BRANCH_L = 0b01100_000000000000000000000000000;
    public static final int END_L0_BRANCH_L = 0b01100_111111111111111111111111111;
    /**
     * {@code branch.ld i27}
     */
    public static final int L0_BRANCH_LD = 0b01101_000000000000000000000000000;
    public static final int END_L0_BRANCH_LD = 0b01101_111111111111111111111111111;
    /**
     * {@code branch.l.ld i27}
     */
    public static final int L0_BRANCH_L_LD = 0b01110_000000000000000000000000000;
    public static final int END_L0_BRANCH_L_LD = 0b01110_111111111111111111111111111;
    /**
     * {@code branch.eq i22 Xcond}
     */
    public static final int L0_BRANCH_EQ = 0b01111_00000_0000000000000000000000;
    public static final int END_L0_BRANCH_EQ = 0b01111_11111_1111111111111111111111;
    /**
     * {@code branch.ne i22 Xcond}
     */
    public static final int L0_BRANCH_NE = 0b10000_00000_0000000000000000000000;
    public static final int END_L0_BRANCH_NE = 0b10000_11111_1111111111111111111111;
    /**
     * {@code branch.lt i22 Xcond}
     */
    public static final int L0_BRANCH_LT = 0b10001_00000_0000000000000000000000;
    public static final int END_L0_BRANCH_LT = 0b10001_11111_1111111111111111111111;
    /**
     * {@code branch.gt i22 Xcond}
     */
    public static final int L0_BRANCH_GT = 0b10010_00000_0000000000000000000000;
    public static final int END_L0_BRANCH_GT = 0b10010_11111_1111111111111111111111;
    /**
     * {@code branch.le i22 Xcond}
     */
    public static final int L0_BRANCH_LE = 0b10011_00000_0000000000000000000000;
    public static final int END_L0_BRANCH_LE = 0b10011_11111_1111111111111111111111;
    /**
     * {@code branch.ge i22 Xcond}
     */
    public static final int L0_BRANCH_GE = 0b10100_00000_0000000000000000000000;
    public static final int END_L0_BRANCH_GE = 0b10100_11111_1111111111111111111111;
    /**
     * {@code branch.zr i22 Xsrc}
     */
    public static final int L0_BRANCH_ZR = 0b10101_00000_0000000000000000000000;
    public static final int END_L0_BRANCH_ZR = 0b10101_11111_1111111111111111111111;
    /**
     * {@code branch.nz i22 Xsrc}
     */
    public static final int L0_BRANCH_NZ = 0b10110_00000_0000000000000000000000;
    public static final int END_L0_BRANCH_NZ = 0b10110_11111_1111111111111111111111;
    /**
     * {@code lea Xdst i22}
     */
    public static final int L0_LEA = 0b10111_0000000000000000000000_00000;
    public static final int END_L0_LEA = 0b10111_1111111111111111111111_11111;
    /**
     * {@code shl Xdst Xlhs u11}
     */
    public static final int L1_SHL = 0b11111_000000_00000000000_00000_00000;
    public static final int END_L1_SHL = 0b11111_000000_11111111111_11111_11111;
    /**
     * {@code shr Xdst Xlhs u11}
     */
    public static final int L1_SHR = 0b11111_000001_00000000000_00000_00000;
    public static final int END_L1_SHR = 0b11111_000001_11111111111_11111_11111;
    /**
     * {@code shrs Xdst Xlhs u11}
     */
    public static final int L1_SHRS = 0b11111_000010_00000000000_00000_00000;
    public static final int END_L1_SHRS = 0b11111_000010_11111111111_11111_11111;
    /**
     * {@code ldr Xdst Xsrc i11}
     */
    public static final int L1_LDR = 0b11111_000011_00000000000_00000_00000;
    public static final int END_L1_LDR = 0b11111_000011_11111111111_11111_11111;
    /**
     * {@code ldrb Xdst Xsrc i11}
     */
    public static final int L1_LDRB = 0b11111_000100_00000000000_00000_00000;
    public static final int END_L1_LDRB = 0b11111_000100_11111111111_11111_11111;
    /**
     * {@code ldrh Xdst Xsrc i11}
     */
    public static final int L1_LDRH = 0b11111_000101_00000000000_00000_00000;
    public static final int END_L1_LDRH = 0b11111_000101_11111111111_11111_11111;
    /**
     * {@code ldrw Xdst Xsrc i11}
     */
    public static final int L1_LDRW = 0b11111_000110_00000000000_00000_00000;
    public static final int END_L1_LDRW = 0b11111_000110_11111111111_11111_11111;
    /**
     * {@code str Xdst Xsrc i11}
     */
    public static final int L1_STR = 0b11111_000111_00000000000_00000_00000;
    public static final int END_L1_STR = 0b11111_000111_11111111111_11111_11111;
    /**
     * {@code strb Xdst Xsrc i11}
     */
    public static final int L1_STRB = 0b11111_001000_00000000000_00000_00000;
    public static final int END_L1_STRB = 0b11111_001000_11111111111_11111_11111;
    /**
     * {@code strh Xdst Xsrc i11}
     */
    public static final int L1_STRH = 0b11111_001001_00000000000_00000_00000;
    public static final int END_L1_STRH = 0b11111_001001_11111111111_11111_11111;
    /**
     * {@code strw Xdst Xsrc i11}
     */
    public static final int L1_STRW = 0b11111_001010_00000000000_00000_00000;
    public static final int END_L1_STRW = 0b11111_001010_11111111111_11111_11111;
    /**
     * {@code int u16}
     */
    public static final int L1_INT = 0b11111_001011_00000_0000000000000000;
    public static final int END_L1_INT = 0b11111_001011_00000_1111111111111111;
    /**
     * {@code ncall u21}
     */
    public static final int L1_NCALL = 0b11111_001100_000000000000000000000;
    public static final int END_L1_NCALL = 0b11111_001100_111111111111111111111;
    /**
     * {@code vcall u21}
     */
    public static final int L1_VCALL = 0b11111_001101_000000000000000000000;
    public static final int END_L1_VCALL = 0b11111_001101_111111111111111111111;
    /**
     * {@code add Xdst Xlhs Xrhs}
     */
    public static final int L2_ADD = 0b11111111111_000000_00000_00000_00000;
    public static final int END_L2_ADD = 0b11111111111_000000_11111_11111_11111;
    /**
     * {@code sub Xdst Xlhs Xrhs}
     */
    public static final int L2_SUB = 0b11111111111_000001_00000_00000_00000;
    public static final int END_L2_SUB = 0b11111111111_000001_11111_11111_11111;
    /**
     * {@code mul Xdst Xlhs Xrhs}
     */
    public static final int L2_MUL = 0b11111111111_000010_00000_00000_00000;
    public static final int END_L2_MUL = 0b11111111111_000010_11111_11111_11111;
    /**
     * {@code div Xdst Xlhs Xrhs}
     */
    public static final int L2_DIV = 0b11111111111_000011_00000_00000_00000;
    public static final int END_L2_DIV = 0b11111111111_000011_11111_11111_11111;
    /**
     * {@code rem Xdst Xlhs Xrhs}
     */
    public static final int L2_REM = 0b11111111111_000100_00000_00000_00000;
    public static final int END_L2_REM = 0b11111111111_000100_11111_11111_11111;
    /**
     * {@code divs Xdst Xlhs Xrhs}
     */
    public static final int L2_DIVS = 0b11111111111_000101_00000_00000_00000;
    public static final int END_L2_DIVS = 0b11111111111_000101_11111_11111_11111;
    /**
     * {@code rems Xdst Xlhs Xrhs}
     */
    public static final int L2_REMS = 0b11111111111_000110_00000_00000_00000;
    public static final int END_L2_REMS = 0b11111111111_000110_11111_11111_11111;
    /**
     * {@code addf Xdst Xlhs Xrhs}
     */
    public static final int L2_ADDF = 0b11111111111_000111_00000_00000_00000;
    public static final int END_L2_ADDF = 0b11111111111_000111_11111_11111_11111;
    /**
     * {@code subf Xdst Xlhs Xrhs}
     */
    public static final int L2_SUBF = 0b11111111111_001000_00000_00000_00000;
    public static final int END_L2_SUBF = 0b11111111111_001000_11111_11111_11111;
    /**
     * {@code mulf Xdst Xlhs Xrhs}
     */
    public static final int L2_MULF = 0b11111111111_001001_00000_00000_00000;
    public static final int END_L2_MULF = 0b11111111111_001001_11111_11111_11111;
    /**
     * {@code divf Xdst Xlhs Xrhs}
     */
    public static final int L2_DIVF = 0b11111111111_001010_00000_00000_00000;
    public static final int END_L2_DIVF = 0b11111111111_001010_11111_11111_11111;
    /**
     * {@code remf Xdst Xlhs Xrhs}
     */
    public static final int L2_REMF = 0b11111111111_001011_00000_00000_00000;
    public static final int END_L2_REMF = 0b11111111111_001011_11111_11111_11111;
    /**
     * {@code and Xdst Xlhs Xrhs}
     */
    public static final int L2_AND = 0b11111111111_001100_00000_00000_00000;
    public static final int END_L2_AND = 0b11111111111_001100_11111_11111_11111;
    /**
     * {@code or Xdst Xlhs Xrhs}
     */
    public static final int L2_OR = 0b11111111111_001101_00000_00000_00000;
    public static final int END_L2_OR = 0b11111111111_001101_11111_11111_11111;
    /**
     * {@code xor Xdst Xlhs Xrhs}
     */
    public static final int L2_XOR = 0b11111111111_001110_00000_00000_00000;
    public static final int END_L2_XOR = 0b11111111111_001110_11111_11111_11111;
    /**
     * {@code shl Xdst Xlhs Xrhs}
     */
    public static final int L2_SHL = 0b11111111111_001111_00000_00000_00000;
    public static final int END_L2_SHL = 0b11111111111_001111_11111_11111_11111;
    /**
     * {@code shr Xdst Xlhs Xrhs}
     */
    public static final int L2_SHR = 0b11111111111_010000_00000_00000_00000;
    public static final int END_L2_SHR = 0b11111111111_010000_11111_11111_11111;
    /**
     * {@code shrs Xdst Xlhs Xrhs}
     */
    public static final int L2_SHRS = 0b11111111111_010001_00000_00000_00000;
    public static final int END_L2_SHRS = 0b11111111111_010001_11111_11111_11111;
    /**
     * {@code cmp Xdst Xlhs Xrhs}
     */
    public static final int L2_CMP = 0b11111111111_010010_00000_00000_00000;
    public static final int END_L2_CMP = 0b11111111111_010010_11111_11111_11111;
    /**
     * {@code cmps Xdst Xlhs Xrhs}
     */
    public static final int L2_CMPS = 0b11111111111_010011_00000_00000_00000;
    public static final int END_L2_CMPS = 0b11111111111_010011_11111_11111_11111;
    /**
     * {@code cmpf Xdst Xlhs Xrhs}
     */
    public static final int L2_CMPF = 0b11111111111_010100_00000_00000_00000;
    public static final int END_L2_CMPF = 0b11111111111_010100_11111_11111_11111;
    /**
     * {@code not Xdst Xsrc}
     */
    public static final int L3_NOT = 0b11111111111111111_00000_00000_00000;
    public static final int END_L3_NOT = 0b11111111111111111_00000_11111_11111;
    /**
     * {@code mov Xdst Xsrc}
     */
    public static final int L3_MOV = 0b11111111111111111_00001_00000_00000;
    public static final int END_L3_MOV = 0b11111111111111111_00001_11111_11111;
    /**
     * {@code fti Xdst Xsrc}
     */
    public static final int L3_FTI = 0b11111111111111111_00010_00000_00000;
    public static final int END_L3_FTI = 0b11111111111111111_00010_11111_11111;
    /**
     * {@code itf Xdst Xsrc}
     */
    public static final int L3_ITF = 0b11111111111111111_00011_00000_00000;
    public static final int END_L3_ITF = 0b11111111111111111_00011_11111_11111;
    /**
     * {@code branch Xdst}
     */
    public static final int L4_BRANCH = 0b1111111111111111111111_00000_00000;
    public static final int END_L4_BRANCH = 0b1111111111111111111111_00000_11111;
    /**
     * {@code branch.l Xdst}
     */
    public static final int L4_BRANCH_L = 0b1111111111111111111111_00001_00000;
    public static final int END_L4_BRANCH_L = 0b1111111111111111111111_00001_11111;
    /**
     * {@code branch.ld Xsrc}
     */
    public static final int L4_BRANCH_LD = 0b1111111111111111111111_00010_00000;
    public static final int END_L4_BRANCH_LD = 0b1111111111111111111111_00010_11111;
    /**
     * {@code branch.l.ld Xsrc}
     */
    public static final int L4_BRANCH_L_LD = 0b1111111111111111111111_00011_00000;
    public static final int END_L4_BRANCH_L_LD = 0b1111111111111111111111_00011_11111;
    /**
     * {@code branch.bo Xdst}
     */
    public static final int L4_BRANCH_BO = 0b1111111111111111111111_00100_00000;
    public static final int END_L4_BRANCH_BO = 0b1111111111111111111111_00100_11111;
    /**
     * {@code branch.l.bo Xdst}
     */
    public static final int L4_BRANCH_L_BO = 0b1111111111111111111111_00101_00000;
    public static final int END_L4_BRANCH_L_BO = 0b1111111111111111111111_00101_11111;
    /**
     * {@code branch.ld.bo Xsrc}
     */
    public static final int L4_BRANCH_LD_BO = 0b1111111111111111111111_00110_00000;
    public static final int END_L4_BRANCH_LD_BO = 0b1111111111111111111111_00110_11111;
    /**
     * {@code branch.bo.ld Xsrc}
     */
    public static final int L4_BRANCH_BO_LD = 0b1111111111111111111111_00111_00000;
    public static final int END_L4_BRANCH_BO_LD = 0b1111111111111111111111_00111_11111;
    /**
     * {@code branch.bo.ld.bo Xsrc}
     */
    public static final int L4_BRANCH_BO_LD_BO = 0b1111111111111111111111_01000_00000;
    public static final int END_L4_BRANCH_BO_LD_BO = 0b1111111111111111111111_01000_11111;
    /**
     * {@code branch.l.ld.bo Xsrc}
     */
    public static final int L4_BRANCH_L_LD_BO = 0b1111111111111111111111_01001_00000;
    public static final int END_L4_BRANCH_L_LD_BO = 0b1111111111111111111111_01001_11111;
    /**
     * {@code branch.l.bo.ld Xsrc}
     */
    public static final int L4_BRANCH_L_BO_LD = 0b1111111111111111111111_01010_00000;
    public static final int END_L4_BRANCH_L_BO_LD = 0b1111111111111111111111_01010_11111;
    /**
     * {@code branch.l.bo.ld.bo Xsrc}
     */
    public static final int L4_BRANCH_L_BO_LD_BO = 0b1111111111111111111111_01011_00000;
    public static final int END_L4_BRANCH_L_BO_LD_BO = 0b1111111111111111111111_01011_11111;
    /**
     * {@code ncall Xid}
     */
    public static final int L4_NCALL = 0b1111111111111111111111_01100_00000;
    public static final int END_L4_NCALL = 0b1111111111111111111111_01100_11111;
    /**
     * {@code vcall Xid}
     */
    public static final int L4_VCALL = 0b1111111111111111111111_01101_00000;
    public static final int END_L4_VCALL = 0b1111111111111111111111_01101_11111;
    /**
     * {@code ldbo Xdst}
     */
    public static final int L4_LDBO = 0b1111111111111111111111_01110_00000;
    public static final int END_L4_LDBO = 0b1111111111111111111111_01110_11111;
    /**
     * {@code ldpc Xdst}
     */
    public static final int L4_LDPC = 0b1111111111111111111111_01111_00000;
    public static final int END_L4_LDPC = 0b1111111111111111111111_01111_11111;
    /**
     * {@code nop}
     */
    public static final int L5_NOP = 0b111111111111111111111111111_00000;
    public static final int END_L5_NOP = 0b111111111111111111111111111_00000;
    /**
     * {@code halt}
     */
    public static final int L5_HALT = 0b111111111111111111111111111_00001;
    public static final int END_L5_HALT = 0b111111111111111111111111111_00001;
    /**
     * {@code ret}
     */
    public static final int L5_RET = 0b111111111111111111111111111_00010;
    public static final int END_L5_RET = 0b111111111111111111111111111_00010;
}
