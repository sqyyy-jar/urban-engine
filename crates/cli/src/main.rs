use std::{
    fs::{self, File},
    path::PathBuf,
};

use clap::{arg, command, crate_version, error::ErrorKind, value_parser, ArgAction, Command};
use urban_common::{binary::Header, opcodes};
use urban_lua::parse;
use urban_runtime::{
    context::{noverify::UnsafeContext, safe::SafeContext, Context},
    vmods::util::Util,
};

const BUILD_DATE: &str = env!("BUILD_DATE");

fn main() {
    let mut cmd = Command::new("urban")
        .bin_name("urban")
        .subcommand_required(true)
        .subcommands([
            command!("version").alias("v"),
            command!("run").alias("r").args([
                arg!([BINARY] "The file to execute")
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
                arg!(--noverify "Execute code in a noverify context").action(ArgAction::SetTrue),
            ]),
            command!("compile").alias("c").args([
                arg!([SOURCE] "The sourcefile to compile")
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
                arg!([OUT] "The output file to write to")
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
            ]),
        ]);
    let matches = cmd.get_matches_mut();
    match matches.subcommand() {
        Some(("version", _)) => {
            println!("urban {} {BUILD_DATE}", crate_version!());
            println!("Implementing Urban ISA version {}", opcodes::VERSION);
        }
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
            let header = Header::read(&content);
            if let Err(err) = header {
                cmd.error(ErrorKind::Format, err).exit();
            };
            let header = header.unwrap();
            if !header.has_valid_magic() {
                cmd.error(ErrorKind::Format, "Binary has invalid byte magic")
                    .exit();
            }
            if !header.is_executable() {
                cmd.error(ErrorKind::Format, "Binary is not executable")
                    .exit();
            }
            if matches.get_flag("noverify") {
                let mut ctx = UnsafeContext::new(
                    (content.as_mut_ptr() as usize + 16) as _,
                    (content.as_mut_ptr() as usize + 16 + header.entrypoint as usize) as _,
                );
                ctx.load_vmod(&Util);
                loop {
                    ctx.decode_instruction()
                }
            } else {
                let mut ctx = SafeContext::new(
                    (content.as_mut_ptr() as usize + 16) as _,
                    (content.as_mut_ptr() as usize + 16 + header.entrypoint as usize) as _,
                    content.len() - 16,
                );
                ctx.load_vmod(&Util);
                while !ctx.has_halted() {
                    ctx.decode_instruction()
                }
            }
        }
        Some(("compile", matches)) => {
            let source = matches.get_one::<PathBuf>("SOURCE").expect("Sourcefile");
            let out = matches.get_one::<PathBuf>("OUT").expect("Outfile");
            if !source.exists() {
                cmd.error(ErrorKind::Io, "Source does not exist").exit();
            }
            if !source.is_file() {
                cmd.error(ErrorKind::Io, "Source is not a file").exit();
            }
            let outfile = File::create(out);
            if let Err(err) = outfile {
                cmd.error(ErrorKind::Io, err).exit();
            }
            let mut outfile = outfile.unwrap();
            let content = fs::read_to_string(source);
            if let Err(err) = content {
                cmd.error(ErrorKind::Io, err).exit();
            }
            let content = content.unwrap();
            if let Err(err) = parse(&content, &mut outfile) {
                cmd.error(ErrorKind::ArgumentConflict, err).exit();
            };
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
