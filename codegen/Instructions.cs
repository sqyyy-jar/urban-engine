using System.Collections.Immutable;
using System.Text;
using System.Text.Json;

namespace urban_codegen;

public class Instructions
{
    public readonly string Version;
    public readonly ImmutableList<Layer> Layers;

    private Instructions(string version, IEnumerable<Layer> layers)
    {
        Version = version;
        Layers = layers.ToImmutableList();
    }

    public static Instructions Load(string path)
    {
        var text = File.ReadAllText(path);
        var root = JsonDocument.Parse(text).RootElement;
        var version = root.GetProperty("version").GetString()!;
        var layers = LoadLayers(root.GetProperty("layers"));
        return new Instructions(version, layers);
    }

    private static IEnumerable<Layer> LoadLayers(JsonElement json)
    {
        var prefixBits = 0u;
        var layers = new List<Layer>();
        foreach (var layer in json.EnumerateArray())
        {
            var bits = layer.GetProperty("bits").GetUInt32();
            var instructions = LoadInstructions(layer.GetProperty("instructions"), prefixBits + bits);
            layers.Add(new Layer(prefixBits, bits, instructions));
            prefixBits += bits;
        }

        return layers;
    }

    private static IEnumerable<Instruction> LoadInstructions(JsonElement json, uint layerBits)
    {
        var instructions = new List<Instruction>();
        var index = 0u;
        foreach (var instruction in json.EnumerateArray())
        {
            var name = instruction.GetProperty("name").GetString()!;
            var components = LoadComponents(instruction.GetProperty("components"));
            var lostBits = 32u - layerBits - (uint)components.Sum(it => it.Bits);
            instructions.Add(new Instruction(name, index++, lostBits, components));
        }

        return instructions;
    }

    private static List<IInstructionComponent> LoadComponents(JsonElement json)
    {
        var components = new List<IInstructionComponent>();
        foreach (var component in json.EnumerateArray())
        {
            var parts = component.GetString()!.Split(":");
            components.Add(parts[0] switch
            {
                "r" => new Register(parts[1]),
                "u" => new UnsignedImmediate(parts[1], uint.Parse(parts[2])),
                "i" => new SignedImmediate(parts[1], uint.Parse(parts[2])),
                _ => throw new Exception($"Invalid component '{component}'"),
            });
        }

        return components;
    }

    public void Verify()
    {
        var usedStates = 0uL;
        var lostBits = 0u;
        var layerId = 0;
        foreach (var layer in Layers)
        {
            var layerBits = layer.PrefixBits + layer.Bits;
            if (layerBits > 32)
            {
                throw new Exception($"Too high amount of layer bits on L{layerId}: {layerBits}");
            }

            var maxInstructionCount = (1u << (int)layer.Bits) - 1;
            if (layerId == Layers.Count - 1)
            {
                maxInstructionCount += 1;
            }

            var instructionCount = layer.Instructions.Count;
            if (instructionCount > maxInstructionCount)
            {
                throw new Exception(
                    $"Too high amount of instructions on L{layerId}: {instructionCount}/{maxInstructionCount}");
            }

            usedStates += (ulong)instructionCount;
            foreach (var instruction in layer.Instructions)
            {
                var usedBits =
                    instruction.Components.Aggregate(layerBits, (current, component) => current + component.Bits);
                if (usedBits > 32)
                {
                    throw new Exception(
                        $"Too high amount of instruction bits on L{layerId}/{instruction.Name}: {usedBits}");
                }

                var componentBits =
                    instruction.Components.Aggregate(0u, (current, component) => current + component.Bits);
                usedStates += (1uL << (int)componentBits) - 1uL;
                lostBits += 32 - usedBits;
                if (usedBits < 32)
                {
                    Console.WriteLine($"{32 - usedBits} lost bits on L{layerId}/{instruction.Name}");
                }
            }

            layerId++;
        }

        Console.WriteLine($"ISA looses {lostBits} bits.");
        Console.WriteLine(
            $"ISA has {usedStates}/{uint.MaxValue + 1uL} ({Math.Round(usedStates / (double)(uint.MaxValue + 1uL) * 100.0)}%) states used.");
    }

    public override string ToString()
    {
        var res = new StringBuilder($"Version: {Version}\n" +
                                    $"Layers: [\n");
        foreach (var layer in Layers)
        {
            res.Append($"{layer},\n");
        }

        res.Append("]");
        return res.ToString();
    }
}

public class Layer
{
    public readonly uint PrefixBits;
    public readonly uint Bits;
    public readonly ImmutableList<Instruction> Instructions;

    public Layer(uint prefixBits, uint bits, IEnumerable<Instruction> instructions)
    {
        PrefixBits = prefixBits;
        Bits = bits;
        Instructions = instructions.ToImmutableList();
    }

    public override string ToString()
    {
        var res = new StringBuilder($"  PrefixBits: {PrefixBits}\n" +
                                    $"  Bits: {Bits}\n" +
                                    $"  Instructions: [\n");
        foreach (var instruction in Instructions)
        {
            res.Append($"{instruction},\n");
        }

        res.Append("  ]");
        return res.ToString();
    }
}

public class Instruction
{
    public readonly string Name;
    public readonly uint Index;
    public readonly uint LostBits;
    public readonly ImmutableList<IInstructionComponent> Components;

    public Instruction(string name, uint index, uint lostBits, IEnumerable<IInstructionComponent> components)
    {
        Name = name;
        Index = index;
        LostBits = lostBits;
        Components = components.ToImmutableList();
    }

    public override string ToString()
    {
        var res = new StringBuilder($"    Name: {Name}\n" +
                                    $"    LostBits: {LostBits}\n" +
                                    $"    Components: [");
        res.Append(string.Join(", ", Components.Select(it => it.DocString)));
        res.Append("]");
        return res.ToString();
    }
}

public interface IInstructionComponent
{
    public string Name { get; }
    public string DocString { get; }
    public uint Bits { get; }
}

public class Register : IInstructionComponent
{
    public string Name { get; }
    public string DocString => $"X{Name}";
    public uint Bits => 5;

    public Register(string name)
    {
        Name = name;
    }
}

public class UnsignedImmediate : IInstructionComponent
{
    public string Name { get; }
    public string DocString => $"u{Bits}";
    public uint Bits { get; }

    public UnsignedImmediate(string name, uint bits)
    {
        Name = name;
        Bits = bits;
    }
}

public class SignedImmediate : IInstructionComponent
{
    public string Name { get; }
    public string DocString => $"i{Bits}";
    public uint Bits { get; }

    public SignedImmediate(string name, uint bits)
    {
        Name = name;
        Bits = bits;
    }
}