import instruction_gen
import interrupt_gen

with open("src/asm.rs", mode="w") as f:
    instruction_gen.write(f)
with open("src/int.rs", mode="w") as f:
    interrupt_gen.write(f)
