use std::{
    collections::HashMap,
    io::{Read, Write},
    process::exit,
    slice,
};

use crate::{
    bus::InstructionBus,
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

    #[inline(always)]
    pub fn check_memory_access(&mut self, address: *mut ()) {
        // TODO global memory access
        if self.mem_base > address as _ {
            self.panic(ERR_ILLEGAL_MEMORY_ACCESS);
        }
        let base_offset = address as usize - self.mem_base as usize;
        if base_offset >= self.mem_size {
            self.panic(ERR_ILLEGAL_MEMORY_ACCESS);
        }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[inline(always)]
    pub fn load<T: Copy>(&mut self, address: *mut T) -> T {
        self.check_memory_access(address as _);
        unsafe { *address }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[inline(always)]
    pub fn store<T: Copy>(&mut self, address: *mut T, value: T) {
        self.check_memory_access(address as _);
        unsafe { *address = value }
    }
}

impl Context for SafeContext {
    fn fetch_instruction(&mut self) -> u32 {
        unsafe { *self.mem }
    }

    fn advance_counter(&mut self) {
        let counter = unsafe { self.mem.add(1) };
        self.check_memory_access(counter as _);
        self.mem = counter;
    }

    fn counter(&mut self) -> *mut u32 {
        self.mem
    }

    fn set_counter(&mut self, counter: *mut u32) {
        self.check_memory_access(counter as _);
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
        for (i, reg) in self.registers.chunks(4).enumerate() {
            eprintln!(
                " R{:<2}: 0x{:016X} | R{:<2}: 0x{:016X} | R{:<2}: 0x{:016X} | R{:<2}: 0x{:016X}",
                i * 4,
                unsafe { reg[0].uint },
                i * 4 + 1,
                unsafe { reg[1].uint },
                i * 4 + 2,
                unsafe { reg[2].uint },
                i * 4 + 3,
                unsafe { reg[3].uint }
            );
        }
        match error_code {
            ERR_ILLEGAL_INSN => {
                eprintln!();
                eprintln!(
                    "Illegal instruction at address {:?}: 0x{:08X}",
                    self.mem,
                    unsafe { *self.mem }
                );
            }
            ERR_ILLEGAL_MEMORY_ACCESS => {
                eprintln!();
                eprintln!(
                    "Illegal memory access in instruction at  address {:?}: 0x{:08X}",
                    self.mem,
                    unsafe { *self.mem }
                );
            }
            _ => {}
        }
        exit(-1)
    }
}

impl InstructionBus for SafeContext {
    #[inline(always)]
    fn l0_add(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint + rhs },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l0_sub(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint - rhs },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l0_mul(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint * rhs },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l0_div(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint / rhs },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l0_adds(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = signed_immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[lhs].int + rhs },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l0_subs(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = signed_immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[lhs].int - rhs },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l0_muls(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = signed_immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[lhs].int * rhs },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l0_divs(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = signed_immediate::<17>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[lhs].int / rhs },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l0_ldr(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let src = signed_immediate::<22>(insn, 5);
        self.registers[dst] = self.load(unsafe { self.mem.offset(src as _) } as *mut _);
        self.advance_counter();
    }

    #[inline(always)]
    fn l0_str(&mut self, insn: u32) {
        let dst = signed_immediate::<22>(insn, 0);
        let src = reg(insn, 22);
        self.store(
            unsafe { self.mem.offset(dst as _) } as *mut _,
            self.registers[src],
        );
        self.advance_counter();
    }

    #[inline(always)]
    fn l0_mov(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let value = immediate::<22>(insn, 5);
        self.registers[dst] = Value { uint: value };
        self.advance_counter();
    }

    #[inline(always)]
    fn l0_movs(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let value = signed_immediate::<22>(insn, 5);
        self.registers[dst] = Value { int: value };
        self.advance_counter();
    }

    #[inline(always)]
    fn l0_branch(&mut self, insn: u32) {
        let imm = signed_immediate::<27>(insn, 0);
        self.set_counter(unsafe { self.mem.offset(imm as _) });
    }

    #[inline(always)]
    fn l0_branch_l(&mut self, insn: u32) {
        let imm = signed_immediate::<27>(insn, 0);
        self.registers[30] = Value {
            size: self.mem as usize + 4,
        };
        self.set_counter(unsafe { self.mem.offset(imm as _) });
    }

    #[inline(always)]
    fn l0_branch_ld(&mut self, insn: u32) {
        let src = signed_immediate::<27>(insn, 0);
        let res = self.load(unsafe { self.mem.offset(src as _) } as *mut _);
        self.set_counter(res);
    }

    #[inline(always)]
    fn l0_branch_l_ld(&mut self, insn: u32) {
        let src = signed_immediate::<27>(insn, 0);
        self.registers[30] = Value {
            size: self.mem as usize + 4,
        };
        let res = self.load(unsafe { self.mem.offset(src as _) } as *mut _);
        self.set_counter(res);
    }

    #[inline(always)]
    fn l0_branch_eq(&mut self, insn: u32) {
        let dst = signed_immediate::<22>(insn, 0);
        let cond = reg(insn, 22);
        if unsafe { self.registers[cond].ord }.is_eq() {
            self.set_counter(unsafe { self.mem.offset(dst as _) });
        } else {
            self.advance_counter();
        }
    }

    #[inline(always)]
    fn l0_branch_ne(&mut self, insn: u32) {
        let dst = signed_immediate::<22>(insn, 0);
        let cond = reg(insn, 22);
        if unsafe { self.registers[cond].ord }.is_ne() {
            self.set_counter(unsafe { self.mem.offset(dst as _) });
        } else {
            self.advance_counter();
        }
    }

    #[inline(always)]
    fn l0_branch_lt(&mut self, insn: u32) {
        let dst = signed_immediate::<22>(insn, 0);
        let cond = reg(insn, 22);
        if unsafe { self.registers[cond].ord }.is_lt() {
            self.set_counter(unsafe { self.mem.offset(dst as _) });
        } else {
            self.advance_counter();
        }
    }

    #[inline(always)]
    fn l0_branch_gt(&mut self, insn: u32) {
        let dst = signed_immediate::<22>(insn, 0);
        let cond = reg(insn, 22);
        if unsafe { self.registers[cond].ord }.is_gt() {
            self.set_counter(unsafe { self.mem.offset(dst as _) });
        } else {
            self.advance_counter();
        }
    }

    #[inline(always)]
    fn l0_branch_le(&mut self, insn: u32) {
        let dst = signed_immediate::<22>(insn, 0);
        let cond = reg(insn, 22);
        if unsafe { self.registers[cond].ord }.is_le() {
            self.set_counter(unsafe { self.mem.offset(dst as _) });
        } else {
            self.advance_counter();
        }
    }

    #[inline(always)]
    fn l0_branch_ge(&mut self, insn: u32) {
        let dst = signed_immediate::<22>(insn, 0);
        let cond = reg(insn, 22);
        if unsafe { self.registers[cond].ord }.is_ge() {
            self.set_counter(unsafe { self.mem.offset(dst as _) });
        } else {
            self.advance_counter();
        }
    }

    #[inline(always)]
    fn l1_shl(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = immediate::<11>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint << rhs },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l1_shr(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = immediate::<11>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint >> rhs },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l1_shrs(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = immediate::<11>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[lhs].int >> rhs },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l1_ldr(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let src = reg(insn, 5);
        let offset = signed_immediate::<11>(insn, 10);
        self.registers[dst] =
            self.load((unsafe { self.registers[src].isize } + offset as isize) as _);
        self.advance_counter();
    }

    #[inline(always)]
    fn l1_ldrb(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let src = reg(insn, 5);
        let offset = signed_immediate::<11>(insn, 10);
        self.registers[dst].uint =
            self.load((unsafe { self.registers[src].isize } + offset as isize) as *mut u8) as u64;
        self.advance_counter();
    }

    #[inline(always)]
    fn l1_ldrh(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let src = reg(insn, 5);
        let offset = signed_immediate::<11>(insn, 10);
        self.registers[dst].uint =
            self.load((unsafe { self.registers[src].isize } + offset as isize) as *mut u16) as u64;
        self.advance_counter();
    }

    #[inline(always)]
    fn l1_ldrw(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let src = reg(insn, 5);
        let offset = signed_immediate::<11>(insn, 10);
        self.registers[dst].uint =
            self.load((unsafe { self.registers[src].isize } + offset as isize) as *mut u32) as u64;
        self.advance_counter();
    }

    #[inline(always)]
    fn l1_str(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let src = reg(insn, 5);
        let offset = signed_immediate::<11>(insn, 10);
        self.store(
            (unsafe { self.registers[dst].isize } + offset as isize) as *mut _,
            self.registers[src],
        );
        self.advance_counter();
    }

    #[inline(always)]
    fn l1_strb(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let src = reg(insn, 5);
        let offset = signed_immediate::<11>(insn, 10);
        self.store(
            (unsafe { self.registers[dst].isize } + offset as isize) as *mut _,
            unsafe { self.registers[src].uint as u8 },
        );
        self.advance_counter();
    }

    #[inline(always)]
    fn l1_strh(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let src = reg(insn, 5);
        let offset = signed_immediate::<11>(insn, 10);
        self.store(
            (unsafe { self.registers[dst].isize } + offset as isize) as *mut _,
            unsafe { self.registers[src].uint as u16 },
        );
        self.advance_counter();
    }

    #[inline(always)]
    fn l1_strw(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let src = reg(insn, 5);
        let offset = signed_immediate::<11>(insn, 10);
        self.store(
            (unsafe { self.registers[dst].isize } + offset as isize) as *mut _,
            unsafe { self.registers[src].uint as u32 },
        );
        self.advance_counter();
    }

    #[inline(always)]
    fn l1_int(&mut self, insn: u32) {
        let id = immediate::<16>(insn, 0) as u16;
        match id {
            INT_READ => {
                let fd = unsafe { self.registers[0].uint };
                let buf = unsafe { self.registers[1].size as *mut u8 };
                self.check_memory_access(buf as _);
                let count = unsafe { self.registers[2].uint };
                self.check_memory_access(unsafe { buf.add(count as _) } as _);
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
                let buf = unsafe { self.registers[1].size as *const u8 };
                self.check_memory_access(buf as _);
                let count = unsafe { self.registers[2].uint };
                self.check_memory_access(unsafe { buf.add(count as _) } as _);
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
    fn l1_n_call(&mut self, insn: u32) {
        let id = immediate::<21>(insn, 0);
        let Some(func) = self.ntable.get(id as usize) else {
            panic!("Tried to ncall position 0x{id:x}");
        };
        func(self);
        self.advance_counter();
    }

    #[inline(always)]
    fn l1_v_call(&mut self, insn: u32) {
        let id = immediate::<21>(insn, 0) as usize;
        let Some(func) = self.vtable.get(&id) else {
            panic!("Tried to vcall id 0x{id:x}");
        };
        func(self);
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_add(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint + self.registers[rhs].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_sub(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint - self.registers[rhs].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_mul(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint * self.registers[rhs].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_div(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint / self.registers[rhs].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_adds(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[lhs].int + self.registers[rhs].int },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_subs(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[lhs].int - self.registers[rhs].int },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_muls(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[lhs].int * self.registers[rhs].int },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_divs(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[lhs].int / self.registers[rhs].int },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_addf(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            float: unsafe { self.registers[lhs].float + self.registers[rhs].float },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_subf(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            float: unsafe { self.registers[lhs].float - self.registers[rhs].float },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_mulf(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            float: unsafe { self.registers[lhs].float * self.registers[rhs].float },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_divf(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            float: unsafe { self.registers[lhs].float / self.registers[rhs].float },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_and(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint & self.registers[rhs].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_or(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint | self.registers[rhs].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_xor(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint ^ self.registers[rhs].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_shl(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint << self.registers[rhs].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_shr(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[lhs].uint >> self.registers[rhs].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_shrs(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[lhs].int >> self.registers[rhs].int },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_cmp(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            ord: unsafe { self.registers[lhs].uint.cmp(&self.registers[rhs].uint) },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_cmps(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            ord: unsafe { self.registers[lhs].int.cmp(&self.registers[rhs].int) },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l2_cmpf(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let lhs = reg(insn, 5);
        let rhs = reg(insn, 10);
        self.registers[dst] = Value {
            ord: unsafe {
                self.registers[lhs]
                    .float
                    .total_cmp(&self.registers[rhs].float)
            },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l3_not(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let src = reg(insn, 5);
        self.registers[dst] = Value {
            uint: unsafe { !self.registers[src].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l3_mov(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let src = reg(insn, 5);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[src].uint },
        };
        self.advance_counter();
    }

    #[inline(always)]
    fn l4_branch(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        self.set_counter(unsafe { self.registers[dst].size } as _);
    }

    #[inline(always)]
    fn l4_branch_l(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        self.registers[30] = Value {
            size: self.mem as usize + 4,
        };
        self.set_counter(unsafe { self.registers[dst].size } as _);
    }

    #[inline(always)]
    fn l4_branch_ld(&mut self, insn: u32) {
        let src = reg(insn, 0);
        let res = self.load(unsafe { self.mem.offset(self.registers[src].isize) } as *mut _);
        self.set_counter(res);
    }

    #[inline(always)]
    fn l4_branch_l_ld(&mut self, insn: u32) {
        let src = reg(insn, 0);
        self.registers[30] = Value {
            size: self.mem as usize + 4,
        };
        let res = self.load(unsafe { self.mem.offset(self.registers[src].isize) } as *mut _);
        self.set_counter(res);
    }

    #[inline(always)]
    fn l5_nop(&mut self, _insn: u32) {
        self.advance_counter();
    }

    #[inline(always)]
    fn l5_halt(&mut self, _insn: u32) {
        self.halted = true;
    }

    #[inline(always)]
    fn unknown(&mut self, _insn: u32) {
        self.panic(ERR_ILLEGAL_INSN);
    }
}
