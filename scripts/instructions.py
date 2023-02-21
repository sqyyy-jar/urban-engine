# TODO
# * Add combination of load and branch to loaded address
# * Add register-based bitshift operations
# * Migrate Python scripts to C#

l1_instructions = [
    {"name": "add", "registers": 2, "size": 17},
    {"name": "sub", "registers": 2, "size": 17},
    {"name": "mul", "registers": 2, "size": 17},
    {"name": "div", "registers": 2, "size": 17},
    {"name": "adds", "registers": 2, "size": -17},
    {"name": "subs", "registers": 2, "size": -17},
    {"name": "muls", "registers": 2, "size": -17},
    {"name": "divs", "registers": 2, "size": -17},
    {"name": "ldr", "registers": 1, "size": -22},
    {"name": "str", "registers": 1, "size": -22},
    {"name": "mov", "registers": 1, "size": 22},
    {"name": "movs", "registers": 1, "size": -22},
    {"name": "b", "registers": 0, "size": -27},
    {"name": "bl", "registers": 0, "size": -27},
    {"name": "b.eq", "registers": 1, "size": -22},
    {"name": "b.ne", "registers": 1, "size": -22},
    {"name": "b.lt", "registers": 1, "size": -22},
    {"name": "b.gt", "registers": 1, "size": -22},
    {"name": "b.le", "registers": 1, "size": -22},
    {"name": "b.ge", "registers": 1, "size": -22},
]

l2_instructions = [
    {"name": "not", "registers": 2, "size": 0},
    {"name": "add", "registers": 3, "size": 0},
    {"name": "sub", "registers": 3, "size": 0},
    {"name": "mul", "registers": 3, "size": 0},
    {"name": "div", "registers": 3, "size": 0},
    {"name": "adds", "registers": 3, "size": 0},
    {"name": "subs", "registers": 3, "size": 0},
    {"name": "muls", "registers": 3, "size": 0},
    {"name": "divs", "registers": 3, "size": 0},
    {"name": "addf", "registers": 3, "size": 0},
    {"name": "subf", "registers": 3, "size": 0},
    {"name": "mulf", "registers": 3, "size": 0},
    {"name": "divf", "registers": 3, "size": 0},
    {"name": "and", "registers": 3, "size": 0},
    {"name": "or", "registers": 3, "size": 0},
    {"name": "xor", "registers": 3, "size": 0},
    {"name": "shl", "registers": 2, "size": 7},
    {"name": "shr", "registers": 2, "size": 7},
    {"name": "shrs", "registers": 2, "size": 7},
    {"name": "ldr", "registers": 2, "size": -11},
    {"name": "ldrb", "registers": 2, "size": -11},
    {"name": "ldrh", "registers": 2, "size": -11},
    {"name": "ldrw", "registers": 2, "size": -11},
    {"name": "str", "registers": 2, "size": -11},
    {"name": "strb", "registers": 2, "size": -11},
    {"name": "strh", "registers": 2, "size": -11},
    {"name": "strw", "registers": 2, "size": -11},
    {"name": "mov", "registers": 2, "size": 0},
    {"name": "br", "registers": 1, "size": 0},
    {"name": "brl", "registers": 1, "size": 0},
    {"name": "nop", "registers": 0, "size": 0},
    {"name": "halt", "registers": 0, "size": 0},
    {"name": "interrupt", "registers": 0, "size": 16},
    {"name": "ncall", "registers": 0, "size": 21},
    {"name": "vcall", "registers": 0, "size": 21},
    {"name": "cmp", "registers": 3, "size": 0},
    {"name": "cmps", "registers": 3, "size": 0},
    {"name": "cmpf", "registers": 3, "size": 0},
]


def map_insn(insn, index: int, layered: bool):
    if insn["size"] < 0:
        insn["size"] = abs(insn["size"])
        insn["signed"] = True
    else:
        insn["signed"] = False
    insn["index"] = index
    insn["layered"] = layered


def mapped():
    l1 = sorted(l1_instructions, key=lambda it: it["name"])
    l2 = sorted(l2_instructions, key=lambda it: it["name"])
    for i, elem in enumerate(l1):
        map_insn(elem, i, False)
    for i, elem in enumerate(l2):
        map_insn(elem, i, True)
    return (l1, l2)
