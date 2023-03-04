use std::collections::HashMap;

use crate::{
    context::{noverify::UnsafeContext, safe::SafeContext},
    vmod::VMod,
};

pub struct Util;

impl VMod<UnsafeContext> for Util {
    fn load(&self, vtable: &mut HashMap<usize, fn(&mut UnsafeContext)>) {
        vtable.insert(0x4200, noverify::uint_to_string);
        vtable.insert(0x4201, noverify::int_to_string);
        vtable.insert(0x4202, noverify::float_to_string);
    }
}

impl VMod<SafeContext> for Util {
    fn load(&self, vtable: &mut HashMap<usize, fn(&mut SafeContext)>) {
        vtable.insert(0x4200, safe::uint_to_string);
        vtable.insert(0x4201, safe::int_to_string);
        vtable.insert(0x4202, safe::float_to_string);
    }
}

mod noverify {
    use crate::context::{noverify::UnsafeContext, Value};

    pub fn uint_to_string(ctx: &mut UnsafeContext) {
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

    pub fn int_to_string(ctx: &mut UnsafeContext) {
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

    pub fn float_to_string(ctx: &mut UnsafeContext) {
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
}

mod safe {
    use crate::context::{safe::SafeContext, Value};

    pub fn uint_to_string(ctx: &mut SafeContext) {
        let value = unsafe { ctx.registers[0].uint };
        let dst = unsafe { (ctx.mem_base as usize + ctx.registers[1].size) as *mut u8 };
        ctx.check_memory_access(dst as _);
        let max_len = unsafe { ctx.registers[2].size };
        ctx.check_memory_access(unsafe { dst.add(max_len) } as _);
        let string = value.to_string();
        if max_len == 0 {
            ctx.registers[0] = Value { int: -1 };
            return;
        }
        let len = string.len().min(max_len);
        unsafe { dst.copy_from_nonoverlapping(string.as_ptr(), string.len().min(max_len)) };
        ctx.registers[0] = Value { size: len };
    }

    pub fn int_to_string(ctx: &mut SafeContext) {
        let value = unsafe { ctx.registers[0].int };
        let dst = unsafe { (ctx.mem_base as usize + ctx.registers[1].size) as *mut u8 };
        ctx.check_memory_access(dst as _);
        let max_len = unsafe { ctx.registers[2].size };
        ctx.check_memory_access(unsafe { dst.add(max_len) } as _);
        let string = value.to_string();
        if max_len == 0 {
            ctx.registers[0] = Value { int: -1 };
            return;
        }
        let len = string.len().min(max_len);
        unsafe { dst.copy_from_nonoverlapping(string.as_ptr(), string.len().min(max_len)) };
        ctx.registers[0] = Value { size: len };
    }

    pub fn float_to_string(ctx: &mut SafeContext) {
        let value = unsafe { ctx.registers[0].float };
        let dst = unsafe { (ctx.mem_base as usize + ctx.registers[1].size) as *mut u8 };
        ctx.check_memory_access(dst as _);
        let max_len = unsafe { ctx.registers[2].size };
        ctx.check_memory_access(unsafe { dst.add(max_len) } as _);
        let string = value.to_string();
        if max_len == 0 {
            ctx.registers[0] = Value { int: -1 };
            return;
        }
        let len = string.len().min(max_len);
        unsafe { dst.copy_from_nonoverlapping(string.as_ptr(), string.len().min(max_len)) };
        ctx.registers[0] = Value { size: len };
    }
}
