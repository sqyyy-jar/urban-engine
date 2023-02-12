from instructions import mapped


def gen_match():
    insns = mapped()
    res = "match insn {\n"
    func_names = [
        insn["name"].replace(".", "_") + ("_imm" if insn["number-arg"] else "")
        for insn in insns
    ]
    const_names = [
        insn["name"].upper().replace(".", "_")
        + ("_IMMEDIATE" if insn["number-arg"] else "")
        for insn in insns
    ]
    for i in range(len(const_names)):
        res += f"    INSN_{const_names[i]}..=ENDINSN_{const_names[i]} => self.{func_names[i]}(insn),\n"
    res += "    _ => {}\n}"
    return res


print(gen_match())
