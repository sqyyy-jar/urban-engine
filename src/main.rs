use std::{fs, path::PathBuf};

use assembler::tokenizer::tokenize;
use clap::{arg, command, error::ErrorKind, value_parser, ArgAction, Command};
use context::{noverify::UnsafeContext, safe::SafeContext, Context};
use vmod::util::Util;

pub mod asm;
pub mod assembler;
pub mod context;
pub mod err;
pub mod int;
pub mod rt;
pub mod vmod;

fn main() {
    let mut cmd = Command::new("urban")
        .bin_name("urban")
        .subcommand_required(true)
        .subcommands([
            command!("run").alias("r").args([
                arg!([BINARY] "The file to execute")
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
                arg!(--noverify "Execute code in a noverify context").action(ArgAction::SetTrue),
            ]),
            command!("compile").alias("c").arg(
                arg!([SOURCE] "The sourcefile to compile")
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
            ),
        ]);
    let matches = cmd.get_matches_mut();
    match matches.subcommand() {
        Some(("run", matches)) => {
            let file = matches.get_one::<PathBuf>("BINARY").expect("No such file");
            if !file.exists() {
                cmd.error(ErrorKind::Io, "Binary does not exist").exit();
            }
            if !file.is_file() {
                cmd.error(ErrorKind::Io, "Binary is not a file").exit();
            }
            let content = fs::read(file);
            if let Err(err) = content {
                cmd.error(ErrorKind::Io, err).exit();
            }
            let mut content = content.unwrap();
            if matches.get_flag("noverify") {
                let mut ctx =
                    UnsafeContext::new(content.as_mut_ptr() as _, content.as_mut_ptr() as _);
                ctx.load_vmod(&Util);
                loop {
                    ctx.decode_instruction()
                }
            } else {
                let mut ctx = SafeContext::new(
                    content.as_mut_ptr() as _,
                    content.as_mut_ptr() as _,
                    content.len(),
                );
                ctx.load_vmod(&Util);
                while !ctx.has_halted() {
                    ctx.decode_instruction()
                }
            }
        }
        Some(("compile", matches)) => {
            let file = matches.get_one::<PathBuf>("SOURCE").expect("No such file");
            if !file.exists() {
                cmd.error(ErrorKind::Io, "Source does not exist").exit();
            }
            if !file.is_file() {
                cmd.error(ErrorKind::Io, "Source is not a file").exit();
            }
            let content = fs::read_to_string(file);
            if let Err(err) = content {
                cmd.error(ErrorKind::Io, err).exit();
            }
            let content = content.unwrap();
            let result = tokenize(&content);
            if let Err(err) = result {
                cmd.error(ErrorKind::Io, err).exit();
            }
            let result = result.unwrap();
            println!("{result:#?}");
            // for token in result {
            //     println!("{}", &content[token.span()])
            // }
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
