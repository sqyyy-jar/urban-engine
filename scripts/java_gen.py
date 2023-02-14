import os
from textwrap import indent

from instructions import mapped


class_name_mappings = {
    "addf": "AddFloat",
    "adds": "AddSigned",
    "b": "Branch",
    "bl": "BranchLinked",
    "br": "BranchRegister",
    "brl": "BranchRegisterLinked",
    "b.eq": "BranchEqual",
    "b.ge": "BranchGreaterEqual",
    "b.gt": "BranchGreater",
    "b.le": "BranchLessEqual",
    "b.lt": "BranchLess",
    "b.ne": "BranchNotEqual",
    "divf": "DivFloat",
    "divs": "DivSigned",
    "ldr": "Load",
    "mov": "Move",
    "movs": "MoveSigned",
    "mulf": "MulFloat",
    "muls": "MulSigned",
    "str": "Store",
    "subf": "SubFloat",
    "subs": "SubSigned",
}
method_name_mappings = {}


def write(insn, layered: bool):
    name = insn["name"]
    size = insn["size"]
    registers = insn["registers"]
    size = insn["size"]
    index = insn["index"]
    class_name = map_class_name(name, size)
    with open(f"gen/{class_name}.java", "w") as file:
        write_class(file, class_name, registers, size, index, layered)


def write_class(
    file, class_name: str, registers: int, size: int, index: int, layered: bool
):
    print(
        f"""package com.github.sqyyy.urban.assembler.insn;

import com.github.sqyyy.urban.assembler.Instruction;

public record {class_name}({gen_fields(registers, size)}) implements Instruction {{
    @Override
    public int write() {{
        int opc = {gen_opc(index, layered)};
{indent(gen_register_modifications(registers, size), " " * 8)}
        return opc;
    }}
}}""",
        file=file,
    )


def gen_fields(registers: int, size: int) -> str:
    res = ""
    for i in range(registers):
        if len(res) > 0:
            res += ", "
        res += f"int reg{i}"
    if size > 0:
        if len(res) > 0:
            res += ", "
        res += f"long immediate"
    return res


def gen_opc(index: int, layered: bool) -> str:
    num = 0
    if layered:
        num |= 0b11111 << 27
        num |= index << 21
    else:
        num |= index << 27
    return hex(num)


def gen_register_modifications(registers: int, size: int) -> str:
    res = ""
    for i in range(registers):
        if registers - i - 1 == 0:
            res += f"opc |= reg{i} & {hex(0b11111)};\n"
            continue
        res += f"opc |= (reg{i} & {hex(0b11111)}) << {(registers - i - 1) * 5};\n"
    if size > 0:
        if registers == 0:
            res += f"opc |= (int) immediate & {hex(0b11111)};\n"
            return res
        res += f"opc |= ((int) immediate & {hex((1 << size) - 1)}) << {registers * 5};\n"
    return res.rstrip()


def map_class_name(name: str, size: int) -> str:
    if name in class_name_mappings:
        if size > 0:
            return class_name_mappings[name] + "Immediate"
        return class_name_mappings[name]
    if size > 0:
        return "".join([x.capitalize() for x in name.split(".")]) + "Immediate"
    return "".join([x.capitalize() for x in name.split(".")])


def map_method_name(name: str) -> str:
    if name in method_name_mappings:
        return method_name_mappings[name]
    parts = name.split(".")
    capitalized = [x.capitalize() for x in parts]
    capitalized[0] = parts[0]
    return "".join(capitalized)


if __name__ == "__main__":
    os.system("mkdir gen")
    os.system("rm -rf gen/**")
    layers = mapped()
    l1 = layers[0]
    l2 = layers[1]
    for insn in l1:
        write(insn, False)
    for insn in l2:
        write(insn, True)
