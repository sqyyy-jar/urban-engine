from instructions import mapped


def gen_insn(value, id):
    name = value["name"]
    registers = value["registers"]
    doc_name = name.replace("_imm", "")
    const_name = name.upper().replace(".", "_").replace("IMM", "IMMEDIATE")
    res = f"/// `{doc_name}"
    for i in range(registers):
        if i != 0:
            res += ","
        res += f" <X{i}>"
    if value["number-arg"]:
        if registers > 0:
            res += ","
        res += f" <i{32 - 6 - registers * 5}>"
    res += "`\npub const INSN_"
    res += const_name
    res += f": u32 = 0b{id:06b}_"
    for _ in range(32 - 6 - registers * 5):
        res += "0"
    for _ in range(registers):
        res += "_00000"
    res += ";\npub const ENDINSN_"
    res += const_name
    res += f": u32 = 0b{id:06b}_"
    for _ in range(32 - 6 - registers * 5):
        res += "1"
    for _ in range(registers):
        res += "_11111"
    res += ";"
    return res


def write(out):
    print(
        """
// This file is automatically generated.
// It is not intended for manual editing.

//! This module contains constants for all instructions
//!
//! ## Instruction
//!
//! 0bXXXXX_000000000000000000000000000
//!
//! `X` represents the id of the instruction

#![allow(clippy::unusual_byte_groupings)]
    """.strip(),
        file=out,
    )
    print(file=out)
    for i, x in enumerate(mapped()):
        print(gen_insn(x, i), file=out)
