﻿using urban_codegen;
using urban_codegen.codegen;

var instructions = Instructions.Load("isa.json");
instructions.Verify();
new Rust().Run(instructions);
new Java().Run(instructions);
new CSharp().Run(instructions);
new Python().Run(instructions);