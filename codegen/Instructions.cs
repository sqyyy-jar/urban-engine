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
            var instructions = LoadInstructions(layer.GetProperty("instructions"));
            layers.Add(new Layer(prefixBits, bits, instructions));
            prefixBits += bits;
        }

        return layers;
    }

    private static IEnumerable<Instruction> LoadInstructions(JsonElement json)
    {
        var instructions = new List<Instruction>();
        foreach (var instruction in json.EnumerateArray())
        {
            var name = instruction.GetProperty("name").GetString()!;
            var components = LoadComponents(instruction.GetProperty("components"));
            instructions.Add(new Instruction(name, components));
        }

        return instructions;
    }

    private static IEnumerable<IInstructionComponent> LoadComponents(JsonElement json)
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
    public readonly ImmutableList<IInstructionComponent> Components;

    public Instruction(string name, IEnumerable<IInstructionComponent> components)
    {
        Name = name;
        Components = components.ToImmutableList();
    }

    public override string ToString()
    {
        var res = new StringBuilder($"    Name: {Name}\n" +
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