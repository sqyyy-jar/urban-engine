use std::{
    collections::HashMap,
    io::{Read, Write},
    process::exit,
    slice,
};

use crate::{
    err::{ERR_ILLEGAL_INSN, ERR_ILLEGAL_MEMORY_ACCESS},
    int::{INT_READ, INT_WRITE},
    vmod::VMod,
};

use super::{immediate, reg, signed_immediate, terminal::Terminal, Context, Value};

pub struct SafeContext {
    pub mem_base: *mut u32,
    pub mem: *mut u32,
    pub mem_size: usize,
    pub registers: [Value; 32],
    pub terminal: Terminal,
    pub ntable: Vec<fn(&mut Self)>,
    pub vtable: HashMap<usize, fn(&mut Self)>,
    pub halted: bool,
}

impl SafeContext {
    pub fn new(mem_base: *mut u32, cursor: *mut u32, mem_size: usize) -> Self {
        Self {
            mem_base,
            mem: cursor,
            mem_size,
            registers: [Value { uint: 0 }; 32],
            terminal: Terminal::default(),
            ntable: Vec::with_capacity(0),
            vtable: HashMap::with_capacity(0),
            halted: false,
        }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[inline(always)]
    pub fn load<T: Copy>(&mut self, address: *mut T) -> T {
        if self.mem_base > address as _ {
            self.panic(ERR_ILLEGAL_MEMORY_ACCESS);
        }
        let base_offset = address as usize - self.mem_base as usize;
        if base_offset >= self.mem_size {
            self.panic(ERR_ILLEGAL_MEMORY_ACCESS);
        }
        unsafe { *address }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[inline(always)]
    pub fn store<T: Copy>(&mut self, address: *mut T, value: T) {
        if self.mem_base > address as _ {
            self.panic(ERR_ILLEGAL_MEMORY_ACCESS);
        }
        let base_offset = address as usize - self.mem_base as usize;
        if base_offset >= self.mem_size {
            self.panic(ERR_ILLEGAL_MEMORY_ACCESS);
        }
        unsafe { *address = value }
    }
}

impl Context for SafeContext {
    fn fetch_instruction(&mut self) -> u32 {
        unsafe { *self.mem }
    }

    fn advance_counter(&mut self) {
        let base_offset = self.mem as usize - self.mem_base as usize;
        if base_offset >= self.mem_size {
            self.panic(ERR_ILLEGAL_MEMORY_ACCESS);
        }
        self.mem = unsafe { self.mem.add(1) };
    }

    fn counter(&mut self) -> *mut u32 {
        self.mem
    }

    fn set_counter(&mut self, counter: *mut u32) {
        if self.mem_base > counter {
            self.panic(ERR_ILLEGAL_MEMORY_ACCESS);
        }
        let base_offset = counter as usize - self.mem_base as usize;
        if base_offset >= self.mem_size {
            self.panic(ERR_ILLEGAL_MEMORY_ACCESS);
        }
        self.mem = counter;
    }

    fn has_halted(&self) -> bool {
        self.halted
    }

    fn load_vmod(&mut self, vmod: &impl VMod<Self>) {
        vmod.load(&mut self.vtable);
    }

    fn panic(&mut self, error_code: u32) -> ! {
        self.halted = true;
        eprintln!("Runtime paniced:");
        for (i, reg) in self.registers.chunks(2).enumerate() {
            eprintln!(
                " R{:02}: 0x{:064X} | R{:02}: 0x{:064X}",
                i * 2,
                unsafe { reg[0].uint },
                i * 2 + 1,
                unsafe { reg[1].uint }
            );
        }
        match error_code {
            ERR_ILLEGAL_INSN => {
                eprintln!();
                eprintln!(
                    "Illegal instruction at address {:?}: 0x{:032X}",
                    self.mem,
                    unsafe { *self.mem }
                );
            }
            ERR_ILLEGAL_MEMORY_ACCESS => {
                eprintln!();
                eprintln!(
                    "Illegal memory access in instruction at  address {:?}: 0x{:032X}",
                    self.mem,
                    unsafe { *self.mem }
                );
            }
            _ => {}
        }
        exit(-1)
    }

    #[inline(always)]
    fn add(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint + self.registers[b].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn add_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint + imm },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn addf(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            float: unsafe { self.registers[a].float + self.registers[b].float },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn adds(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int + self.registers[b].int },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn adds_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int + imm },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn and(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint & self.registers[b].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn b_imm(&mut self, insn: u32) {
        let imm = signed_immediate::<27>(insn, 0);
        self.set_counter(unsafe { self.mem.offset(imm as _) });
    }

    #[inline(always)]
    fn b_eq_imm(&mut self, insn: u32) {
        let a = reg(insn, 0);
        let imm = signed_immediate::<22>(insn, 5);
        if unsafe { self.registers[a].ord }.is_eq() {
            self.set_counter(unsafe { self.mem.offset(imm as _) });
        } else {
            self.advance_counter();
        }
    }

    #[inline(always)]
    fn b_ge_imm(&mut self, insn: u32) {
        let a = reg(insn, 0);
        let imm = signed_immediate::<22>(insn, 5);
        if unsafe { self.registers[a].ord }.is_ge() {
            self.set_counter(unsafe { self.mem.offset(imm as _) });
        } else {
            self.advance_counter();
        }
    }

    #[inline(always)]
    fn b_gt_imm(&mut self, insn: u32) {
        let a = reg(insn, 0);
        let imm = signed_immediate::<22>(insn, 5);
        if unsafe { self.registers[a].ord }.is_gt() {
            self.set_counter(unsafe { self.mem.offset(imm as _) });
        } else {
            self.advance_counter();
        }
    }

    #[inline(always)]
    fn b_le_imm(&mut self, insn: u32) {
        let a = reg(insn, 0);
        let imm = signed_immediate::<22>(insn, 5);
        if unsafe { self.registers[a].ord }.is_le() {
            self.set_counter(unsafe { self.mem.offset(imm as _) });
        } else {
            self.advance_counter();
        }
    }

    #[inline(always)]
    fn b_lt_imm(&mut self, insn: u32) {
        let a = reg(insn, 0);
        let imm = signed_immediate::<22>(insn, 5);
        if unsafe { self.registers[a].ord }.is_lt() {
            self.set_counter(unsafe { self.mem.offset(imm as _) });
        } else {
            self.advance_counter();
        }
    }

    #[inline(always)]
    fn b_ne_imm(&mut self, insn: u32) {
        let a = reg(insn, 0);
        let imm = signed_immediate::<22>(insn, 5);
        if unsafe { self.registers[a].ord }.is_ne() {
            self.set_counter(unsafe { self.mem.offset(imm as _) });
        } else {
            self.advance_counter();
        }
    }

    #[inline(always)]
    fn bl_imm(&mut self, insn: u32) {
        let imm = signed_immediate::<27>(insn, 0);
        self.registers[30] = Value {
            uint: (self.mem as usize - self.mem_base as usize + 4) as _,
        };
        self.set_counter(unsafe { self.mem.offset(imm as _) });
    }

    #[inline(always)]
    fn br(&mut self, insn: u32) {
        let a = reg(insn, 0);
        self.set_counter(
            (self.mem_base as usize + unsafe { self.registers[a].uint } as usize) as _,
        );
    }

    #[inline(always)]
    fn brl(&mut self, insn: u32) {
        let a = reg(insn, 0);
        self.registers[30] = Value {
            uint: (self.mem as usize - self.mem_base as usize + 4) as _,
        };
        self.set_counter(
            (self.mem_base as usize + unsafe { self.registers[a].uint } as usize) as _,
        );
    }

    #[inline(always)]
    fn cmp(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            ord: unsafe { self.registers[a].uint.cmp(&self.registers[b].uint) },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn cmpf(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            ord: unsafe { self.registers[a].int.cmp(&self.registers[b].int) },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn cmps(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            ord: unsafe { self.registers[a].float.total_cmp(&self.registers[b].float) },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn div(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint / self.registers[b].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn div_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint / imm },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn divf(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            float: unsafe { self.registers[a].float / self.registers[b].float },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn divs(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int / self.registers[b].int },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn divs_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int / imm },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn halt(&mut self, _insn: u32) {
        println!("Halted");
        exit(0);
    }

    #[inline(always)]
    fn interrupt_imm(&mut self, insn: u32) {
        // TODO safety
        let imm = immediate::<16>(insn, 0) as u16;
        match imm {
            INT_READ => {
                let fd = unsafe { self.registers[0].uint };
                let buf = unsafe { (self.mem_base as usize + self.registers[1].size) as *mut u8 };
                let count = unsafe { self.registers[2].uint };
                let res = match fd {
                    0 => self
                        .terminal
                        .stdin
                        .read(unsafe { slice::from_raw_parts_mut(buf, count as _) }),
                    _ => {
                        self.registers[0] = Value { int: -1 };
                        self.advance_counter();
                        return;
                    }
                };
                match res {
                    Ok(count) => {
                        self.registers[0] = Value { size: count };
                    }
                    Err(_) => {
                        self.registers[0] = Value { int: -1 };
                    }
                }
                self.advance_counter();
            }
            INT_WRITE => {
                let fd = unsafe { self.registers[0].uint };
                let buf = unsafe { (self.mem_base as usize + self.registers[1].size) as *const u8 };
                let count = unsafe { self.registers[2].uint };
                let res = match fd {
                    1 => self
                        .terminal
                        .stdout
                        .write(unsafe { slice::from_raw_parts(buf, count as _) }),
                    2 => self
                        .terminal
                        .stderr
                        .write(unsafe { slice::from_raw_parts(buf, count as _) }),
                    _ => {
                        self.registers[0] = Value { int: -1 };
                        self.advance_counter();
                        return;
                    }
                };
                match res {
                    Ok(count) => {
                        self.registers[0] = Value { size: count };
                    }
                    Err(_) => {
                        self.registers[0] = Value { int: -1 };
                    }
                }
                self.advance_counter();
            }
            _ => {
                self.advance_counter();
            }
        }
    }

    #[inline(always)]
    fn ldr(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<11>(insn, 10);
        self.registers[dst] = self.load(
            (self.mem_base as isize + unsafe { self.registers[a].isize } + imm as isize) as _,
        );
        self.advance_counter();
    }

    #[inline(always)]
    fn ldr_byte(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<11>(insn, 10);
        self.registers[dst].uint = self.load(
            (self.mem_base as isize + unsafe { self.registers[a].isize } + imm as isize) as *mut u8,
        ) as u64;
        self.advance_counter();
    }

    #[inline(always)]
    fn ldr_half(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<11>(insn, 10);
        self.registers[dst].uint = self.load(
            (self.mem_base as isize + unsafe { self.registers[a].isize } + imm as isize)
                as *mut u16,
        ) as u64;
        self.advance_counter();
    }

    #[inline(always)]
    fn ldr_word(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<11>(insn, 10);
        self.registers[dst].uint = self.load(
            (self.mem_base as isize + unsafe { self.registers[a].isize } + imm as isize)
                as *mut u32,
        ) as u64;
        self.advance_counter();
    }

    #[inline(always)]
    fn ldr_imm(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let imm = signed_immediate::<22>(insn, 5);
        self.registers[dst] = self.load(unsafe { self.mem.offset(imm as _) } as *mut _);
        self.advance_counter();
    }

    #[inline(always)]
    fn mov(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        self.registers[dst] = self.registers[a];
        self.advance_counter();
    }

    #[inline(always)]
    fn mov_imm(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let imm = immediate::<22>(insn, 5);
        self.registers[dst] = Value { uint: imm };
        self.advance_counter();
    }

    #[inline(always)]
    fn movs_imm(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let imm = signed_immediate::<22>(insn, 5);
        self.registers[dst] = Value { int: imm };
        self.advance_counter();
    }

    #[inline(always)]
    fn mul(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint * self.registers[b].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn mul_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint * imm },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn mulf(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            float: unsafe { self.registers[a].float * self.registers[b].float },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn muls(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int * self.registers[b].int },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn muls_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int * imm },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn nop(&mut self, _insn: u32) {
        self.advance_counter();
    }

    #[inline(always)]
    fn not(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { !self.registers[a].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn or(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint | self.registers[b].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn shl_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<7>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint << imm },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn shr_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<7>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint >> imm },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn shrs_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<7>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int >> imm },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn str(&mut self, insn: u32) {
        // TODO
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<11>(insn, 10);
        self.store(
            (self.mem_base as isize + unsafe { self.registers[dst].isize } + imm as isize)
                as *mut _,
            self.registers[a],
        );
        self.advance_counter();
    }

    #[inline(always)]
    fn str_byte(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<11>(insn, 10);
        self.store(
            (self.mem_base as isize + unsafe { self.registers[dst].isize } + imm as isize)
                as *mut u8,
            unsafe { self.registers[a].uint } as u8,
        );
        self.advance_counter();
    }

    #[inline(always)]
    fn str_half(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<11>(insn, 10);
        self.store(
            (self.mem_base as isize + unsafe { self.registers[dst].isize } + imm as isize)
                as *mut u16,
            unsafe { self.registers[a].uint } as u16,
        );
        self.advance_counter();
    }

    #[inline(always)]
    fn str_word(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<11>(insn, 10);
        self.store(
            (self.mem_base as isize + unsafe { self.registers[dst].isize } + imm as isize)
                as *mut u32,
            unsafe { self.registers[a].uint } as u32,
        );
        self.advance_counter();
    }

    #[inline(always)]
    fn str_imm(&mut self, insn: u32) {
        let a = reg(insn, 0);
        let imm = signed_immediate::<22>(insn, 5);
        self.store(
            unsafe { self.mem.offset(imm as _) } as *mut _,
            self.registers[a],
        );
        self.advance_counter();
    }

    #[inline(always)]
    fn sub(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint - self.registers[b].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn sub_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint - imm },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn subf(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            float: unsafe { self.registers[a].float - self.registers[b].float },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn subs(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int - self.registers[b].int },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn subs_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int + imm },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn xor(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint ^ self.registers[b].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn ncall_imm(&mut self, insn: u32) {
        let imm = immediate::<21>(insn, 0);
        let Some(func) = self.ntable.get(imm as usize) else {
            panic!("Tried to ncall position 0x{imm:x}");
        };
        func(self);
        self.advance_counter();
    }

    #[inline(always)]
    fn vcall_imm(&mut self, insn: u32) {
        let imm = immediate::<21>(insn, 0) as usize;
        let Some(func) = self.vtable.get(&imm) else {
            panic!("Tried to vcall id 0x{imm:x}");
        };
        func(self);
        self.advance_counter();
    }
}
