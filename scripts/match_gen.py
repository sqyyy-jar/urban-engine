from instructions import mapped
from instruction_gen import mangle_name
from trait_function_gen import mangle_function_name

if __name__ == "__main__":
    layers = mapped()
    insns = layers[0]
    insns.extend(layers[1])
    print("match insn {")
    for insn in insns:
        mangled_name = mangle_name(insn["name"], insn["size"], insn["layered"])
        function_name = mangle_function_name(insn["name"], insn["size"], insn["layered"])
        print(
            f"    INSN_{mangled_name}..=ENDINSN_{mangled_name} => self.{function_name}(insn),"
        )
    print("    _ => {")
    print("        panic!()")
    print("    }")
    print("}")
