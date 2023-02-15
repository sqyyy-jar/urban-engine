use std::collections::HashMap;

use crate::context::{noverify::UnsafeContext, Value};

use super::VMod;

pub struct Util;

impl VMod<UnsafeContext> for Util {
    fn load(&self, vtable: &mut HashMap<usize, fn(&mut UnsafeContext)>) {
        vtable.insert(0x4200, uint_to_string);
        vtable.insert(0x4201, int_to_string);
        vtable.insert(0x4202, float_to_string);
    }
}

fn uint_to_string(ctx: &mut UnsafeContext) {
    let value = unsafe { ctx.registers[0].uint };
    let dst = unsafe { (ctx.mem_base as usize + ctx.registers[1].size) as *mut u8 };
    let max_len = unsafe { ctx.registers[2].size };
    let string = value.to_string();
    if max_len == 0 {
        ctx.registers[0] = Value { int: -1 };
        return;
    }
    let len = string.len().min(max_len);
    unsafe { dst.copy_from_nonoverlapping(string.as_ptr(), string.len().min(max_len)) };
    ctx.registers[0] = Value { size: len };
}

fn int_to_string(ctx: &mut UnsafeContext) {
    let value = unsafe { ctx.registers[0].int };
    let dst = unsafe { (ctx.mem_base as usize + ctx.registers[1].size) as *mut u8 };
    let max_len = unsafe { ctx.registers[2].size };
    let string = value.to_string();
    if max_len == 0 {
        ctx.registers[0] = Value { int: -1 };
        return;
    }
    let len = string.len().min(max_len);
    unsafe { dst.copy_from_nonoverlapping(string.as_ptr(), string.len().min(max_len)) };
    ctx.registers[0] = Value { size: len };
}

fn float_to_string(ctx: &mut UnsafeContext) {
    let value = unsafe { ctx.registers[0].float };
    let dst = unsafe { (ctx.mem_base as usize + ctx.registers[1].size) as *mut u8 };
    let max_len = unsafe { ctx.registers[2].size };
    let string = value.to_string();
    if max_len == 0 {
        ctx.registers[0] = Value { int: -1 };
        return;
    }
    let len = string.len().min(max_len);
    unsafe { dst.copy_from_nonoverlapping(string.as_ptr(), string.len().min(max_len)) };
    ctx.registers[0] = Value { size: len };
}
