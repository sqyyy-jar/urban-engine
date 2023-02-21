use std::io::{Error, ErrorKind, Result};

use full_moon::{
    ast::{punctuated::Punctuated, Call, Expression, FunctionArgs, FunctionCall, Suffix, Value},
    tokenizer::TokenType,
};
use phf::{phf_map, Map};

use super::{Binary, ConstValue};

type Return = Result<ConstValue>;

/// Functions that can be called in the root level
pub const ROOT_FUNCS: Map<&'static str, fn(&mut Binary, &FunctionCall)> = phf_map! {
    // "" => |_| {},
};
/// Functions that can be called in constants in the root level
pub const ROOT_ASSIGN_FUNCS: Map<&'static str, fn(&mut Binary, &FunctionCall) -> Return> = phf_map! {
    "u" => uint,
};

fn collect_args(call: &FunctionCall) -> &Punctuated<Expression> {
    let mut args = call.suffixes();
    let arg = args.next().expect("Suffix");
    if args.next().is_some() {
        panic!("More than one suffix");
    }
    match arg {
        Suffix::Call(call) => {
            let args = match call {
                Call::AnonymousCall(args) => args,
                Call::MethodCall(call) => call.args(),
                _ => unreachable!(),
            };
            match args {
                FunctionArgs::Parentheses { arguments, .. } => arguments,
                _ => panic!("Expected parantheses call"),
            }
        }
        _ => panic!("Expected direct call"),
    }
}

fn uint(_: &mut Binary, call: &FunctionCall) -> Result<ConstValue> {
    let arguments = collect_args(call);
    if arguments.len() > 1 {
        return Err(Error::new(ErrorKind::Other, "More than one arg"));
    }
    let arg = arguments.iter().next().unwrap();
    match arg {
        Expression::Value { value } => match value.as_ref() {
            Value::Number(num) => {
                let TokenType::Number { text } = num.token_type() else {
                    unreachable!();
                };
                Ok(ConstValue::UInt {
                    value: text.parse().expect("uint"),
                })
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
