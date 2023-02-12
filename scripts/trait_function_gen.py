from scripts.instructions import mapped


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


for i, x in enumerate(mapped()):
    print(gen_trait_functions(x, i))
