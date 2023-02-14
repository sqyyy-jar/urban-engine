from sys import stdout
from instructions import mapped
from instruction_gen import gen_documentation


def mangle_function_name(name: str, size: int) -> str:
    if size > 0:
        return f"{name.replace('.', '_')}_imm"
    else:
        return name.replace(".", "_")


def gen_trait_functions(insn):
    name = insn["name"]
    registers = insn["registers"]
    size = insn["size"]
    signed = insn["signed"]
    function_name = mangle_function_name(name, size)
    gen_documentation(stdout, name, registers, size, signed)
    print(f"fn {function_name}(&mut self, insn: u32);\n")


if __name__ == "__main__":
    layers = mapped()
    insns = layers[0]
    insns.extend(layers[1])
    for insn in insns:
        gen_trait_functions(insn)
