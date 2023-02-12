from instructions import mapped


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


print(gen_match(mapped()))
