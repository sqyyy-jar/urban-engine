from instructions import mapped


def gen_trait_functions(value):
    name = value["name"]
    registers = value["registers"]
    number_arg = value["number-arg"]
    res = f"    /// `{name}"
    for i in range(registers):
        if i != 0:
            res += ","
        res += f" <X{i}>"
    if number_arg:
        if registers > 0:
            res += ","
        res += f" <i{32 - 6 - registers * 5}>"
    res += "`\n    fn "
    res += name.replace(".", "_")
    if number_arg:
        res += "_imm"
    res += "(&mut self, insn: u32);\n"
    return res


for x in mapped():
    print(gen_trait_functions(x))
