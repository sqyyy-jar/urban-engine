using urban_codegen;

var instructions = Instructions.Load("isa.json");
instructions.Verify();
Console.WriteLine(instructions);