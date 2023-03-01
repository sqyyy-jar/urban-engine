using System.Text;

namespace urban_codegen.codegen;

public class Rust : Codegen
{
    public void Run(Instructions instructions)
    {
        Directory.CreateDirectory("gen/rust");
        OpCodes(instructions);
        BusTrait(instructions);
    }

    private static void OpCodes(Instructions instructions)
    {
        var opcodesFile = File.Create("gen/rust/opcodes.rs");
        var opcodes = new StreamWriter(opcodesFile);
        opcodes.WriteLine($"""
            // This file is automatically generated.
            // It is not intended for manual editing.

            //! This module contains opcode constants for ISA version `{instructions.Version}`.

            #![allow(clippy::unusual_byte_groupings)]

            """);
        var layerId = 0;
        foreach (var layer in instructions.Layers)
        {
            foreach (var instruction in layer.Instructions)
            {
                var name = instruction.Name.ToUpper().Replace('.', '_');
                opcodes.Write($"/// `{instruction.Name}");
                foreach (var component in instruction.Components)
                {
                    opcodes.Write($" {component.DocString}");
                }

                opcodes.WriteLine("`");
                opcodes.Write($"pub const L{layerId}_{name}: u32 = 0b");
                var startBuilder = new StringBuilder();
                startBuilder.Append(string.Concat(Enumerable.Repeat('1', (int)layer.PrefixBits)));
                if (layer.PrefixBits > 0)
                {
                    startBuilder.Append('_');
                }

                var index = Convert.ToString(instruction.Index, 2);
                startBuilder.Append(string.Concat(Enumerable.Repeat('0', (int)layer.Bits - index.Length)));
                startBuilder.Append(index);
                if (instruction.LostBits > 0)
                {
                    startBuilder.Append('_');
                    startBuilder.Append(string.Concat(Enumerable.Repeat('0', (int)instruction.LostBits)));
                }

                var start = startBuilder.ToString();
                opcodes.Write(start);
                foreach (var component in instruction.Components)
                {
                    opcodes.Write('_');
                    opcodes.Write(string.Concat(Enumerable.Repeat('0', (int)component.Bits)));
                }

                opcodes.WriteLine(";");
                opcodes.Write($"pub const END_L{layerId}_{name}: u32 = 0b");
                opcodes.Write(start);
                foreach (var component in instruction.Components)
                {
                    opcodes.Write('_');
                    opcodes.Write(string.Concat(Enumerable.Repeat('1', (int)component.Bits)));
                }

                opcodes.WriteLine(";");
            }

            layerId++;
        }

        opcodes.Flush();
        opcodesFile.Close();
    }

    private static void BusTrait(Instructions instructions)
    {
        var instructionBusFile = File.Create("gen/rust/instruction_bus.rs");
        var instructionBus = new StreamWriter(instructionBusFile);
        instructionBus.WriteLine($"""
            // This file is automatically generated.
            // It is not intended for manual editing.

            //! This module contains an instruction bus trait for ISA version `{instructions.Version}`.

            use crate::opcodes::*;

            """);
        instructionBus.WriteLine("pub trait InstructionBus {");
        var layerId = 0;
        foreach (var layer in instructions.Layers)
        {
            foreach (var instruction in layer.Instructions)
            {
                var name = instruction.Name.Replace('.', '_');
                instructionBus.Write($"    /// `{instruction.Name}");
                foreach (var component in instruction.Components)
                {
                    instructionBus.Write($" {component.DocString}");
                }

                instructionBus.WriteLine("`");
                instructionBus.WriteLine($"    fn l{layerId}_{name}(&mut self, insn: u32);");
                instructionBus.WriteLine();
            }

            layerId++;
        }

        instructionBus.WriteLine("""
    fn unknown(&mut self, insn: u32);

    fn process(&mut self, insn: u32) {
        match insn {
""");
        layerId = 0;
        foreach (var layer in instructions.Layers)
        {
            foreach (var instruction in layer.Instructions)
            {
                var name = instruction.Name.ToUpper().Replace('.', '_');
                var funcName = instruction.Name.Replace('.', '_');
                instructionBus.WriteLine($"            L{layerId}_{name}..=END_L{layerId}_{name} => self.l{layerId}_{funcName}(insn),");
            }

            layerId++;
        }
        instructionBus.WriteLine("""
            _ => self.unknown(insn),
        }
    }
""");
        instructionBus.WriteLine("}");
        instructionBus.Flush();
        instructionBusFile.Close();
    }
}