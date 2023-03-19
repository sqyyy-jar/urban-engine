// This file is automatically generated.
// It is not intended for manual editing.

/// This module contains opcode constants for ISA version <c>1.2.0-pre</c>.
public class OpCodes
{
    /// <c>add Xdst Xlhs u17</c>
    public static readonly uint L0Add = 0b00000_00000000000000000_00000_00000;
    public static readonly uint EndL0Add = 0b00000_11111111111111111_11111_11111;
    /// <c>sub Xdst Xlhs u17</c>
    public static readonly uint L0Sub = 0b00001_00000000000000000_00000_00000;
    public static readonly uint EndL0Sub = 0b00001_11111111111111111_11111_11111;
    /// <c>mul Xdst Xlhs u17</c>
    public static readonly uint L0Mul = 0b00010_00000000000000000_00000_00000;
    public static readonly uint EndL0Mul = 0b00010_11111111111111111_11111_11111;
    /// <c>div Xdst Xlhs u17</c>
    public static readonly uint L0Div = 0b00011_00000000000000000_00000_00000;
    public static readonly uint EndL0Div = 0b00011_11111111111111111_11111_11111;
    /// <c>rem Xdst Xlhs u17</c>
    public static readonly uint L0Rem = 0b00100_00000000000000000_00000_00000;
    public static readonly uint EndL0Rem = 0b00100_11111111111111111_11111_11111;
    /// <c>divs Xdst Xlhs i17</c>
    public static readonly uint L0Divs = 0b00101_00000000000000000_00000_00000;
    public static readonly uint EndL0Divs = 0b00101_11111111111111111_11111_11111;
    /// <c>rems Xdst Xlhs i17</c>
    public static readonly uint L0Rems = 0b00110_00000000000000000_00000_00000;
    public static readonly uint EndL0Rems = 0b00110_11111111111111111_11111_11111;
    /// <c>ldr Xdst i22</c>
    public static readonly uint L0Ldr = 0b00111_0000000000000000000000_00000;
    public static readonly uint EndL0Ldr = 0b00111_1111111111111111111111_11111;
    /// <c>str i22 Xsrc</c>
    public static readonly uint L0Str = 0b01000_00000_0000000000000000000000;
    public static readonly uint EndL0Str = 0b01000_11111_1111111111111111111111;
    /// <c>mov Xdst u22</c>
    public static readonly uint L0Mov = 0b01001_0000000000000000000000_00000;
    public static readonly uint EndL0Mov = 0b01001_1111111111111111111111_11111;
    /// <c>movs Xdst i22</c>
    public static readonly uint L0Movs = 0b01010_0000000000000000000000_00000;
    public static readonly uint EndL0Movs = 0b01010_1111111111111111111111_11111;
    /// <c>branch i27</c>
    public static readonly uint L0Branch = 0b01011_000000000000000000000000000;
    public static readonly uint EndL0Branch = 0b01011_111111111111111111111111111;
    /// <c>branch.l i27</c>
    public static readonly uint L0BranchL = 0b01100_000000000000000000000000000;
    public static readonly uint EndL0BranchL = 0b01100_111111111111111111111111111;
    /// <c>branch.ld i27</c>
    public static readonly uint L0BranchLd = 0b01101_000000000000000000000000000;
    public static readonly uint EndL0BranchLd = 0b01101_111111111111111111111111111;
    /// <c>branch.l.ld i27</c>
    public static readonly uint L0BranchLLd = 0b01110_000000000000000000000000000;
    public static readonly uint EndL0BranchLLd = 0b01110_111111111111111111111111111;
    /// <c>branch.eq i22 Xcond</c>
    public static readonly uint L0BranchEq = 0b01111_00000_0000000000000000000000;
    public static readonly uint EndL0BranchEq = 0b01111_11111_1111111111111111111111;
    /// <c>branch.ne i22 Xcond</c>
    public static readonly uint L0BranchNe = 0b10000_00000_0000000000000000000000;
    public static readonly uint EndL0BranchNe = 0b10000_11111_1111111111111111111111;
    /// <c>branch.lt i22 Xcond</c>
    public static readonly uint L0BranchLt = 0b10001_00000_0000000000000000000000;
    public static readonly uint EndL0BranchLt = 0b10001_11111_1111111111111111111111;
    /// <c>branch.gt i22 Xcond</c>
    public static readonly uint L0BranchGt = 0b10010_00000_0000000000000000000000;
    public static readonly uint EndL0BranchGt = 0b10010_11111_1111111111111111111111;
    /// <c>branch.le i22 Xcond</c>
    public static readonly uint L0BranchLe = 0b10011_00000_0000000000000000000000;
    public static readonly uint EndL0BranchLe = 0b10011_11111_1111111111111111111111;
    /// <c>branch.ge i22 Xcond</c>
    public static readonly uint L0BranchGe = 0b10100_00000_0000000000000000000000;
    public static readonly uint EndL0BranchGe = 0b10100_11111_1111111111111111111111;
    /// <c>branch.zr i22 Xsrc</c>
    public static readonly uint L0BranchZr = 0b10101_00000_0000000000000000000000;
    public static readonly uint EndL0BranchZr = 0b10101_11111_1111111111111111111111;
    /// <c>branch.nz i22 Xsrc</c>
    public static readonly uint L0BranchNz = 0b10110_00000_0000000000000000000000;
    public static readonly uint EndL0BranchNz = 0b10110_11111_1111111111111111111111;
    /// <c>shl Xdst Xlhs u11</c>
    public static readonly uint L1Shl = 0b11111_000000_00000000000_00000_00000;
    public static readonly uint EndL1Shl = 0b11111_000000_11111111111_11111_11111;
    /// <c>shr Xdst Xlhs u11</c>
    public static readonly uint L1Shr = 0b11111_000001_00000000000_00000_00000;
    public static readonly uint EndL1Shr = 0b11111_000001_11111111111_11111_11111;
    /// <c>shrs Xdst Xlhs u11</c>
    public static readonly uint L1Shrs = 0b11111_000010_00000000000_00000_00000;
    public static readonly uint EndL1Shrs = 0b11111_000010_11111111111_11111_11111;
    /// <c>ldr Xdst Xsrc i11</c>
    public static readonly uint L1Ldr = 0b11111_000011_00000000000_00000_00000;
    public static readonly uint EndL1Ldr = 0b11111_000011_11111111111_11111_11111;
    /// <c>ldrb Xdst Xsrc i11</c>
    public static readonly uint L1Ldrb = 0b11111_000100_00000000000_00000_00000;
    public static readonly uint EndL1Ldrb = 0b11111_000100_11111111111_11111_11111;
    /// <c>ldrh Xdst Xsrc i11</c>
    public static readonly uint L1Ldrh = 0b11111_000101_00000000000_00000_00000;
    public static readonly uint EndL1Ldrh = 0b11111_000101_11111111111_11111_11111;
    /// <c>ldrw Xdst Xsrc i11</c>
    public static readonly uint L1Ldrw = 0b11111_000110_00000000000_00000_00000;
    public static readonly uint EndL1Ldrw = 0b11111_000110_11111111111_11111_11111;
    /// <c>str Xdst Xsrc i11</c>
    public static readonly uint L1Str = 0b11111_000111_00000000000_00000_00000;
    public static readonly uint EndL1Str = 0b11111_000111_11111111111_11111_11111;
    /// <c>strb Xdst Xsrc i11</c>
    public static readonly uint L1Strb = 0b11111_001000_00000000000_00000_00000;
    public static readonly uint EndL1Strb = 0b11111_001000_11111111111_11111_11111;
    /// <c>strh Xdst Xsrc i11</c>
    public static readonly uint L1Strh = 0b11111_001001_00000000000_00000_00000;
    public static readonly uint EndL1Strh = 0b11111_001001_11111111111_11111_11111;
    /// <c>strw Xdst Xsrc i11</c>
    public static readonly uint L1Strw = 0b11111_001010_00000000000_00000_00000;
    public static readonly uint EndL1Strw = 0b11111_001010_11111111111_11111_11111;
    /// <c>int u16</c>
    public static readonly uint L1Int = 0b11111_001011_00000_0000000000000000;
    public static readonly uint EndL1Int = 0b11111_001011_00000_1111111111111111;
    /// <c>ncall u21</c>
    public static readonly uint L1Ncall = 0b11111_001100_000000000000000000000;
    public static readonly uint EndL1Ncall = 0b11111_001100_111111111111111111111;
    /// <c>vcall u21</c>
    public static readonly uint L1Vcall = 0b11111_001101_000000000000000000000;
    public static readonly uint EndL1Vcall = 0b11111_001101_111111111111111111111;
    /// <c>add Xdst Xlhs Xrhs</c>
    public static readonly uint L2Add = 0b11111111111_000000_00000_00000_00000;
    public static readonly uint EndL2Add = 0b11111111111_000000_11111_11111_11111;
    /// <c>sub Xdst Xlhs Xrhs</c>
    public static readonly uint L2Sub = 0b11111111111_000001_00000_00000_00000;
    public static readonly uint EndL2Sub = 0b11111111111_000001_11111_11111_11111;
    /// <c>mul Xdst Xlhs Xrhs</c>
    public static readonly uint L2Mul = 0b11111111111_000010_00000_00000_00000;
    public static readonly uint EndL2Mul = 0b11111111111_000010_11111_11111_11111;
    /// <c>div Xdst Xlhs Xrhs</c>
    public static readonly uint L2Div = 0b11111111111_000011_00000_00000_00000;
    public static readonly uint EndL2Div = 0b11111111111_000011_11111_11111_11111;
    /// <c>rem Xdst Xlhs Xrhs</c>
    public static readonly uint L2Rem = 0b11111111111_000100_00000_00000_00000;
    public static readonly uint EndL2Rem = 0b11111111111_000100_11111_11111_11111;
    /// <c>divs Xdst Xlhs Xrhs</c>
    public static readonly uint L2Divs = 0b11111111111_000101_00000_00000_00000;
    public static readonly uint EndL2Divs = 0b11111111111_000101_11111_11111_11111;
    /// <c>rems Xdst Xlhs Xrhs</c>
    public static readonly uint L2Rems = 0b11111111111_000110_00000_00000_00000;
    public static readonly uint EndL2Rems = 0b11111111111_000110_11111_11111_11111;
    /// <c>addf Xdst Xlhs Xrhs</c>
    public static readonly uint L2Addf = 0b11111111111_000111_00000_00000_00000;
    public static readonly uint EndL2Addf = 0b11111111111_000111_11111_11111_11111;
    /// <c>subf Xdst Xlhs Xrhs</c>
    public static readonly uint L2Subf = 0b11111111111_001000_00000_00000_00000;
    public static readonly uint EndL2Subf = 0b11111111111_001000_11111_11111_11111;
    /// <c>mulf Xdst Xlhs Xrhs</c>
    public static readonly uint L2Mulf = 0b11111111111_001001_00000_00000_00000;
    public static readonly uint EndL2Mulf = 0b11111111111_001001_11111_11111_11111;
    /// <c>divf Xdst Xlhs Xrhs</c>
    public static readonly uint L2Divf = 0b11111111111_001010_00000_00000_00000;
    public static readonly uint EndL2Divf = 0b11111111111_001010_11111_11111_11111;
    /// <c>remf Xdst Xlhs Xrhs</c>
    public static readonly uint L2Remf = 0b11111111111_001011_00000_00000_00000;
    public static readonly uint EndL2Remf = 0b11111111111_001011_11111_11111_11111;
    /// <c>and Xdst Xlhs Xrhs</c>
    public static readonly uint L2And = 0b11111111111_001100_00000_00000_00000;
    public static readonly uint EndL2And = 0b11111111111_001100_11111_11111_11111;
    /// <c>or Xdst Xlhs Xrhs</c>
    public static readonly uint L2Or = 0b11111111111_001101_00000_00000_00000;
    public static readonly uint EndL2Or = 0b11111111111_001101_11111_11111_11111;
    /// <c>xor Xdst Xlhs Xrhs</c>
    public static readonly uint L2Xor = 0b11111111111_001110_00000_00000_00000;
    public static readonly uint EndL2Xor = 0b11111111111_001110_11111_11111_11111;
    /// <c>shl Xdst Xlhs Xrhs</c>
    public static readonly uint L2Shl = 0b11111111111_001111_00000_00000_00000;
    public static readonly uint EndL2Shl = 0b11111111111_001111_11111_11111_11111;
    /// <c>shr Xdst Xlhs Xrhs</c>
    public static readonly uint L2Shr = 0b11111111111_010000_00000_00000_00000;
    public static readonly uint EndL2Shr = 0b11111111111_010000_11111_11111_11111;
    /// <c>shrs Xdst Xlhs Xrhs</c>
    public static readonly uint L2Shrs = 0b11111111111_010001_00000_00000_00000;
    public static readonly uint EndL2Shrs = 0b11111111111_010001_11111_11111_11111;
    /// <c>cmp Xdst Xlhs Xrhs</c>
    public static readonly uint L2Cmp = 0b11111111111_010010_00000_00000_00000;
    public static readonly uint EndL2Cmp = 0b11111111111_010010_11111_11111_11111;
    /// <c>cmps Xdst Xlhs Xrhs</c>
    public static readonly uint L2Cmps = 0b11111111111_010011_00000_00000_00000;
    public static readonly uint EndL2Cmps = 0b11111111111_010011_11111_11111_11111;
    /// <c>cmpf Xdst Xlhs Xrhs</c>
    public static readonly uint L2Cmpf = 0b11111111111_010100_00000_00000_00000;
    public static readonly uint EndL2Cmpf = 0b11111111111_010100_11111_11111_11111;
    /// <c>not Xdst Xsrc</c>
    public static readonly uint L3Not = 0b11111111111111111_00000_00000_00000;
    public static readonly uint EndL3Not = 0b11111111111111111_00000_11111_11111;
    /// <c>mov Xdst Xsrc</c>
    public static readonly uint L3Mov = 0b11111111111111111_00001_00000_00000;
    public static readonly uint EndL3Mov = 0b11111111111111111_00001_11111_11111;
    /// <c>fti Xdst Xsrc</c>
    public static readonly uint L3Fti = 0b11111111111111111_00010_00000_00000;
    public static readonly uint EndL3Fti = 0b11111111111111111_00010_11111_11111;
    /// <c>itf Xdst Xsrc</c>
    public static readonly uint L3Itf = 0b11111111111111111_00011_00000_00000;
    public static readonly uint EndL3Itf = 0b11111111111111111_00011_11111_11111;
    /// <c>branch Xdst</c>
    public static readonly uint L4Branch = 0b1111111111111111111111_00000_00000;
    public static readonly uint EndL4Branch = 0b1111111111111111111111_00000_11111;
    /// <c>branch.l Xdst</c>
    public static readonly uint L4BranchL = 0b1111111111111111111111_00001_00000;
    public static readonly uint EndL4BranchL = 0b1111111111111111111111_00001_11111;
    /// <c>branch.ld Xsrc</c>
    public static readonly uint L4BranchLd = 0b1111111111111111111111_00010_00000;
    public static readonly uint EndL4BranchLd = 0b1111111111111111111111_00010_11111;
    /// <c>branch.l.ld Xsrc</c>
    public static readonly uint L4BranchLLd = 0b1111111111111111111111_00011_00000;
    public static readonly uint EndL4BranchLLd = 0b1111111111111111111111_00011_11111;
    /// <c>branch.bo Xdst</c>
    public static readonly uint L4BranchBo = 0b1111111111111111111111_00100_00000;
    public static readonly uint EndL4BranchBo = 0b1111111111111111111111_00100_11111;
    /// <c>branch.l.bo Xdst</c>
    public static readonly uint L4BranchLBo = 0b1111111111111111111111_00101_00000;
    public static readonly uint EndL4BranchLBo = 0b1111111111111111111111_00101_11111;
    /// <c>branch.ld.bo Xsrc</c>
    public static readonly uint L4BranchLdBo = 0b1111111111111111111111_00110_00000;
    public static readonly uint EndL4BranchLdBo = 0b1111111111111111111111_00110_11111;
    /// <c>branch.bo.ld Xsrc</c>
    public static readonly uint L4BranchBoLd = 0b1111111111111111111111_00111_00000;
    public static readonly uint EndL4BranchBoLd = 0b1111111111111111111111_00111_11111;
    /// <c>branch.bo.ld.bo Xsrc</c>
    public static readonly uint L4BranchBoLdBo = 0b1111111111111111111111_01000_00000;
    public static readonly uint EndL4BranchBoLdBo = 0b1111111111111111111111_01000_11111;
    /// <c>branch.l.ld.bo Xsrc</c>
    public static readonly uint L4BranchLLdBo = 0b1111111111111111111111_01001_00000;
    public static readonly uint EndL4BranchLLdBo = 0b1111111111111111111111_01001_11111;
    /// <c>branch.l.bo.ld Xsrc</c>
    public static readonly uint L4BranchLBoLd = 0b1111111111111111111111_01010_00000;
    public static readonly uint EndL4BranchLBoLd = 0b1111111111111111111111_01010_11111;
    /// <c>branch.l.bo.ld.bo Xsrc</c>
    public static readonly uint L4BranchLBoLdBo = 0b1111111111111111111111_01011_00000;
    public static readonly uint EndL4BranchLBoLdBo = 0b1111111111111111111111_01011_11111;
    /// <c>ncall Xid</c>
    public static readonly uint L4Ncall = 0b1111111111111111111111_01100_00000;
    public static readonly uint EndL4Ncall = 0b1111111111111111111111_01100_11111;
    /// <c>vcall Xid</c>
    public static readonly uint L4Vcall = 0b1111111111111111111111_01101_00000;
    public static readonly uint EndL4Vcall = 0b1111111111111111111111_01101_11111;
    /// <c>ldbo Xdst</c>
    public static readonly uint L4Ldbo = 0b1111111111111111111111_01110_00000;
    public static readonly uint EndL4Ldbo = 0b1111111111111111111111_01110_11111;
    /// <c>ldpc Xdst</c>
    public static readonly uint L4Ldpc = 0b1111111111111111111111_01111_00000;
    public static readonly uint EndL4Ldpc = 0b1111111111111111111111_01111_11111;
    /// <c>nop</c>
    public static readonly uint L5Nop = 0b111111111111111111111111111_00000;
    public static readonly uint EndL5Nop = 0b111111111111111111111111111_00000;
    /// <c>halt</c>
    public static readonly uint L5Halt = 0b111111111111111111111111111_00001;
    public static readonly uint EndL5Halt = 0b111111111111111111111111111_00001;
}
