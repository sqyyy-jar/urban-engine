from instructions import mapped
from instruction_gen import mangle_name
from trait_function_gen import mangle_function_name

if __name__ == "__main__":
    insns = mapped()
    l1 = insns[0]
    l2 = insns[1]
    l1.extend(l2)
    print("match insn {")
    for insn in l1:
        mangled_name = mangle_name(insn["name"], insn["size"])
        function_name = mangle_function_name(insn["name"], insn["size"])
        print(
            f"    INSN_{mangled_name}..=ENDINSN_{mangled_name} => self.{function_name}(insn),"
        )
    print("    _ => {")
    print("        panic!()")
    print("    }")
    print("}")
