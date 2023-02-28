// This file is automatically generated.
// It is not intended for manual editing.

/// This module contains opcode constants for ISA version <c>1.0</c>.
public class OpCodes
{
    /// <c>add Xdst Xlhs u17</c>
    public static readonly uint L0Add = 0b00000_00000_00000_00000000000000000;
    public static readonly uint EndL0Add = 0b00000_11111_11111_11111111111111111;
    /// <c>sub Xdst Xlhs u17</c>
    public static readonly uint L0Sub = 0b00001_00000_00000_00000000000000000;
    public static readonly uint EndL0Sub = 0b00001_11111_11111_11111111111111111;
    /// <c>mul Xdst Xlhs u17</c>
    public static readonly uint L0Mul = 0b00010_00000_00000_00000000000000000;
    public static readonly uint EndL0Mul = 0b00010_11111_11111_11111111111111111;
    /// <c>div Xdst Xlhs u17</c>
    public static readonly uint L0Div = 0b00011_00000_00000_00000000000000000;
    public static readonly uint EndL0Div = 0b00011_11111_11111_11111111111111111;
    /// <c>adds Xdst Xlhs i17</c>
    public static readonly uint L0Adds = 0b00100_00000_00000_00000000000000000;
    public static readonly uint EndL0Adds = 0b00100_11111_11111_11111111111111111;
    /// <c>subs Xdst Xlhs i17</c>
    public static readonly uint L0Subs = 0b00101_00000_00000_00000000000000000;
    public static readonly uint EndL0Subs = 0b00101_11111_11111_11111111111111111;
    /// <c>muls Xdst Xlhs i17</c>
    public static readonly uint L0Muls = 0b00110_00000_00000_00000000000000000;
    public static readonly uint EndL0Muls = 0b00110_11111_11111_11111111111111111;
    /// <c>divs Xdst Xlhs i17</c>
    public static readonly uint L0Divs = 0b00111_00000_00000_00000000000000000;
    public static readonly uint EndL0Divs = 0b00111_11111_11111_11111111111111111;
    /// <c>ldr Xdst i22</c>
    public static readonly uint L0Ldr = 0b01000_00000_0000000000000000000000;
    public static readonly uint EndL0Ldr = 0b01000_11111_1111111111111111111111;
    /// <c>str i22 Xsrc</c>
    public static readonly uint L0Str = 0b01001_0000000000000000000000_00000;
    public static readonly uint EndL0Str = 0b01001_1111111111111111111111_11111;
    /// <c>mov Xdst u22</c>
    public static readonly uint L0Mov = 0b01010_00000_0000000000000000000000;
    public static readonly uint EndL0Mov = 0b01010_11111_1111111111111111111111;
    /// <c>movs Xdst i22</c>
    public static readonly uint L0Movs = 0b01011_00000_0000000000000000000000;
    public static readonly uint EndL0Movs = 0b01011_11111_1111111111111111111111;
    /// <c>branch i27</c>
    public static readonly uint L0Branch = 0b01100_000000000000000000000000000;
    public static readonly uint EndL0Branch = 0b01100_111111111111111111111111111;
    /// <c>branch.l i27</c>
    public static readonly uint L0BranchL = 0b01101_000000000000000000000000000;
    public static readonly uint EndL0BranchL = 0b01101_111111111111111111111111111;
    /// <c>branch.ld i27</c>
    public static readonly uint L0BranchLd = 0b01110_000000000000000000000000000;
    public static readonly uint EndL0BranchLd = 0b01110_111111111111111111111111111;
    /// <c>branch.l.ld i27</c>
    public static readonly uint L0BranchLLd = 0b01111_000000000000000000000000000;
    public static readonly uint EndL0BranchLLd = 0b01111_111111111111111111111111111;
    /// <c>branch.eq i22 Xcond</c>
    public static readonly uint L0BranchEq = 0b10000_0000000000000000000000_00000;
    public static readonly uint EndL0BranchEq = 0b10000_1111111111111111111111_11111;
    /// <c>branch.ne i22 Xcond</c>
    public static readonly uint L0BranchNe = 0b10001_0000000000000000000000_00000;
    public static readonly uint EndL0BranchNe = 0b10001_1111111111111111111111_11111;
    /// <c>branch.lt i22 Xcond</c>
    public static readonly uint L0BranchLt = 0b10010_0000000000000000000000_00000;
    public static readonly uint EndL0BranchLt = 0b10010_1111111111111111111111_11111;
    /// <c>branch.gt i22 Xcond</c>
    public static readonly uint L0BranchGt = 0b10011_0000000000000000000000_00000;
    public static readonly uint EndL0BranchGt = 0b10011_1111111111111111111111_11111;
    /// <c>branch.le i22 Xcond</c>
    public static readonly uint L0BranchLe = 0b10100_0000000000000000000000_00000;
    public static readonly uint EndL0BranchLe = 0b10100_1111111111111111111111_11111;
    /// <c>branch.ge i22 Xcond</c>
    public static readonly uint L0BranchGe = 0b10101_0000000000000000000000_00000;
    public static readonly uint EndL0BranchGe = 0b10101_1111111111111111111111_11111;
    /// <c>shl Xdst Xlhs u11</c>
    public static readonly uint L1Shl = 0b11111_000000_00000_00000_00000000000;
    public static readonly uint EndL1Shl = 0b11111_000000_11111_11111_11111111111;
    /// <c>shr Xdst Xlhs u11</c>
    public static readonly uint L1Shr = 0b11111_000001_00000_00000_00000000000;
    public static readonly uint EndL1Shr = 0b11111_000001_11111_11111_11111111111;
    /// <c>shrs Xdst Xlhs u11</c>
    public static readonly uint L1Shrs = 0b11111_000010_00000_00000_00000000000;
    public static readonly uint EndL1Shrs = 0b11111_000010_11111_11111_11111111111;
    /// <c>ldr Xdst Xsrc i11</c>
    public static readonly uint L1Ldr = 0b11111_000011_00000_00000_00000000000;
    public static readonly uint EndL1Ldr = 0b11111_000011_11111_11111_11111111111;
    /// <c>ldrb Xdst Xsrc i11</c>
    public static readonly uint L1Ldrb = 0b11111_000100_00000_00000_00000000000;
    public static readonly uint EndL1Ldrb = 0b11111_000100_11111_11111_11111111111;
    /// <c>ldrh Xdst Xsrc i11</c>
    public static readonly uint L1Ldrh = 0b11111_000101_00000_00000_00000000000;
    public static readonly uint EndL1Ldrh = 0b11111_000101_11111_11111_11111111111;
    /// <c>ldrw Xdst Xsrc i11</c>
    public static readonly uint L1Ldrw = 0b11111_000110_00000_00000_00000000000;
    public static readonly uint EndL1Ldrw = 0b11111_000110_11111_11111_11111111111;
    /// <c>str Xdst Xsrc i11</c>
    public static readonly uint L1Str = 0b11111_000111_00000_00000_00000000000;
    public static readonly uint EndL1Str = 0b11111_000111_11111_11111_11111111111;
    /// <c>strb Xdst Xsrc i11</c>
    public static readonly uint L1Strb = 0b11111_001000_00000_00000_00000000000;
    public static readonly uint EndL1Strb = 0b11111_001000_11111_11111_11111111111;
    /// <c>strh Xdst Xsrc i11</c>
    public static readonly uint L1Strh = 0b11111_001001_00000_00000_00000000000;
    public static readonly uint EndL1Strh = 0b11111_001001_11111_11111_11111111111;
    /// <c>strw Xdst Xsrc i11</c>
    public static readonly uint L1Strw = 0b11111_001010_00000_00000_00000000000;
    public static readonly uint EndL1Strw = 0b11111_001010_11111_11111_11111111111;
    /// <c>int u16</c>
    public static readonly uint L1Int = 0b11111_001011_00000_0000000000000000;
    public static readonly uint EndL1Int = 0b11111_001011_00000_1111111111111111;
    /// <c>n_call u21</c>
    public static readonly uint L1NCall = 0b11111_001100_000000000000000000000;
    public static readonly uint EndL1NCall = 0b11111_001100_111111111111111111111;
    /// <c>v_call u21</c>
    public static readonly uint L1VCall = 0b11111_001101_000000000000000000000;
    public static readonly uint EndL1VCall = 0b11111_001101_111111111111111111111;
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
    /// <c>adds Xdst Xlhs Xrhs</c>
    public static readonly uint L2Adds = 0b11111111111_000100_00000_00000_00000;
    public static readonly uint EndL2Adds = 0b11111111111_000100_11111_11111_11111;
    /// <c>subs Xdst Xlhs Xrhs</c>
    public static readonly uint L2Subs = 0b11111111111_000101_00000_00000_00000;
    public static readonly uint EndL2Subs = 0b11111111111_000101_11111_11111_11111;
    /// <c>muls Xdst Xlhs Xrhs</c>
    public static readonly uint L2Muls = 0b11111111111_000110_00000_00000_00000;
    public static readonly uint EndL2Muls = 0b11111111111_000110_11111_11111_11111;
    /// <c>divs Xdst Xlhs Xrhs</c>
    public static readonly uint L2Divs = 0b11111111111_000111_00000_00000_00000;
    public static readonly uint EndL2Divs = 0b11111111111_000111_11111_11111_11111;
    /// <c>addf Xdst Xlhs Xrhs</c>
    public static readonly uint L2Addf = 0b11111111111_001000_00000_00000_00000;
    public static readonly uint EndL2Addf = 0b11111111111_001000_11111_11111_11111;
    /// <c>subf Xdst Xlhs Xrhs</c>
    public static readonly uint L2Subf = 0b11111111111_001001_00000_00000_00000;
    public static readonly uint EndL2Subf = 0b11111111111_001001_11111_11111_11111;
    /// <c>mulf Xdst Xlhs Xrhs</c>
    public static readonly uint L2Mulf = 0b11111111111_001010_00000_00000_00000;
    public static readonly uint EndL2Mulf = 0b11111111111_001010_11111_11111_11111;
    /// <c>divf Xdst Xlhs Xrhs</c>
    public static readonly uint L2Divf = 0b11111111111_001011_00000_00000_00000;
    public static readonly uint EndL2Divf = 0b11111111111_001011_11111_11111_11111;
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
    /// <c>branch Xdst</c>
    public static readonly uint L4Branch = 0b1111111111111111111111_00000_00000;
    public static readonly uint EndL4Branch = 0b1111111111111111111111_00000_11111;
    /// <c>branch.l Xdst</c>
    public static readonly uint L4BranchL = 0b1111111111111111111111_00001_00000;
    public static readonly uint EndL4BranchL = 0b1111111111111111111111_00001_11111;
    /// <c>branch.ld Xdst</c>
    public static readonly uint L4BranchLd = 0b1111111111111111111111_00010_00000;
    public static readonly uint EndL4BranchLd = 0b1111111111111111111111_00010_11111;
    /// <c>branch.l.ld Xdst</c>
    public static readonly uint L4BranchLLd = 0b1111111111111111111111_00011_00000;
    public static readonly uint EndL4BranchLLd = 0b1111111111111111111111_00011_11111;
    /// <c>nop</c>
    public static readonly uint L5Nop = 0b111111111111111111111111111_00000;
    public static readonly uint EndL5Nop = 0b111111111111111111111111111_00000;
    /// <c>halt</c>
    public static readonly uint L5Halt = 0b111111111111111111111111111_00001;
    public static readonly uint EndL5Halt = 0b111111111111111111111111111_00001;
}
