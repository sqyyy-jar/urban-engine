from sys import stdout
from instructions import mapped
from instruction_gen import gen_documentation


name_mappings = {
    "ldrb": "ldr_byte",
    "ldrh": "ldr_half",
    "ldrw": "ldr_word",
    "strb": "str_byte",
    "strh": "str_half",
    "strw": "str_word",
}
layer2_name_mappings = {
    "ldr": "ldr",
    "str": "str",
}


def mangle_function_name(name: str, size: int, layered: bool) -> str:
    if name in name_mappings:
        return name_mappings[name]
    if layered and name in layer2_name_mappings:
        return layer2_name_mappings[name]
    if size > 0:
        return f"{name.replace('.', '_')}_imm"
    else:
        return name.replace(".", "_")


def gen_trait_functions(insn):
    name = insn["name"]
    registers = insn["registers"]
    size = insn["size"]
    signed = insn["signed"]
    layered = insn["layered"]
    function_name = mangle_function_name(name, size, layered)
    gen_documentation(stdout, name, registers, size, signed)
    print(f"fn {function_name}(&mut self, insn: u32);\n")


if __name__ == "__main__":
    layers = mapped()
    insns = layers[0]
    insns.extend(layers[1])
    for insn in insns:
        gen_trait_functions(insn)
