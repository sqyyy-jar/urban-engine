pub mod noverify;
pub mod safe;
pub mod terminal;

use std::{cmp::Ordering, fmt::Debug};

use crate::{bus::InstructionBus, vmod::VMod};

#[derive(Clone, Copy)]
pub union Value {
    pub size: usize,
    pub isize: isize,
    pub uint: u64,
    pub int: i64,
    pub float: f64,
    pub ord: Ordering,
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(unsafe { self.uint.to_string().as_str() })
    }
}

pub trait Context: Sized + InstructionBus {
    fn fetch_instruction(&mut self) -> u32;

    fn advance_counter(&mut self);

    fn counter(&mut self) -> *mut u32;

    fn set_counter(&mut self, counter: *mut u32);

    fn has_halted(&self) -> bool;

    fn load_vmod(&mut self, vmod: &impl VMod<Self>);

    fn panic(&mut self, error_code: u32) -> !;

    #[inline(always)]
    fn decode_instruction(&mut self) {
        let insn = self.fetch_instruction();
        self.process(insn);
    }
}

#[inline(always)]
pub const fn reg(insn: u32, bit_pos: usize) -> usize {
    (insn as usize >> bit_pos) & 0x1F
}

#[inline(always)]
pub const fn immediate<const BITS: usize>(insn: u32, bit_pos: usize) -> u64 {
    (insn as u64 >> bit_pos) & ((1 << BITS) - 1)
}

#[inline(always)]
pub const fn signed_immediate<const BITS: usize>(insn: u32, bit_pos: usize) -> i64 {
    let value = (insn >> bit_pos) & ((1 << BITS) - 1);
    if ((value >> (BITS - 1)) & 1) != 0 {
        (value | (!0) << BITS) as i32 as _
    } else {
        value as _
    }
}
