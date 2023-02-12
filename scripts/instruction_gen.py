import pprint

instructions = {
    "not": {"registers": 2, "number-arg": False},
    "add": {"registers": 3, "number-arg": False},
    "add_imm": {"registers": 2, "number-arg": True},
    "sub": {"registers": 3, "number-arg": False},
    "sub_imm": {"registers": 2, "number-arg": True},
    "mul": {"registers": 3, "number-arg": False},
    "mul_imm": {"registers": 2, "number-arg": True},
    "div": {"registers": 3, "number-arg": False},
    "div_imm": {"registers": 2, "number-arg": True},
    "adds": {"registers": 3, "number-arg": False},
    "adds_imm": {"registers": 2, "number-arg": True},
    "subs": {"registers": 3, "number-arg": False},
    "subs_imm": {"registers": 2, "number-arg": True},
    "muls": {"registers": 3, "number-arg": False},
    "muls_imm": {"registers": 2, "number-arg": True},
    "divs": {"registers": 3, "number-arg": False},
    "divs_imm": {"registers": 2, "number-arg": True},
    "addf": {"registers": 3, "number-arg": False},
    "subf": {"registers": 3, "number-arg": False},
    "mulf": {"registers": 3, "number-arg": False},
    "divf": {"registers": 3, "number-arg": False},
    "and": {"registers": 3, "number-arg": False},
    "or": {"registers": 3, "number-arg": False},
    "xor": {"registers": 3, "number-arg": False},
    "shl": {"registers": 2, "number-arg": True},
    "shr": {"registers": 2, "number-arg": True},
    "shrs": {"registers": 2, "number-arg": True},
    "ldr": {"registers": 2, "number-arg": False},
    "ldr_imm": {"registers": 1, "number-arg": True},
    "str": {"registers": 2, "number-arg": False},
    "str_imm": {"registers": 1, "number-arg": True},
    "mov": {"registers": 2, "number-arg": False},
    "mov_imm": {"registers": 1, "number-arg": True},
    "movs_imm": {"registers": 1, "number-arg": True},
    "br": {"registers": 1, "number-arg": False},
    "b_imm": {"registers": 0, "number-arg": True},
    "brl": {"registers": 1, "number-arg": False},
    "bl_imm": {"registers": 0, "number-arg": True},
    "b.eq_imm": {"registers": 1, "number-arg": True},
    "b.nq_imm": {"registers": 1, "number-arg": True},
    "b.lt_imm": {"registers": 1, "number-arg": True},
    "b.gt_imm": {"registers": 1, "number-arg": True},
    "b.le_imm": {"registers": 1, "number-arg": True},
    "b.ge_imm": {"registers": 1, "number-arg": True},
    "nop": {"registers": 0, "number-arg": False},
    "halt": {"registers": 0, "number-arg": False},
    "interrupt_imm": {"registers": 0, "number-arg": True},
}


def scan():
    all = []
    for key, value in instructions.items():
        value["name"] = key
        all.append(value)
    return sorted(all, key=lambda it: it["name"])


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


def gen_trait_functions(value, id):
    name = value["name"]
    registers = value["registers"]
    doc_name = name.replace("_imm", "")
    res = f"    /// `{doc_name}"
    for i in range(registers):
        if i != 0:
            res += ","
        res += f" <X{i}>"
    if value["number-arg"]:
        if registers > 0:
            res += ","
        res += f" <i{32 - 6 - registers * 5}>"
    res += "`\n    fn "
    res += name.replace(".", "_")
    res += "(&mut self, insn: u32);\n"
    return res


def gen_match(scan_result):
    res = "match insn {\n"
    func_names = [insn["name"].replace(".", "_") for insn in scan_result]
    const_names = [
        insn["name"].upper().replace(".", "_").replace("IMM", "IMMEDIATE")
        for insn in scan_result
    ]
    for i in range(len(const_names)):
        res += f"    INSN_{const_names[i]}..=ENDINSN_{const_names[i]} => self.{func_names[i]}(insn),\n"
    res += "    _ => {}\n}"
    return res


res = scan()
# for i, x in enumerate(res):
#     print(gen_insn(x, i))

for i, x in enumerate(res):
    print(gen_trait_functions(x, i))

# print(gen_match(res))
