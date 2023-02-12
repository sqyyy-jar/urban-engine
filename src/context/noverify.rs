use std::process::exit;

use crate::stack::noverify::UnsafeStack;

use super::{immediate, reg, signed_immediate, Context, Value};

pub struct UnsafeContext {
    pub mem_base: *mut u32,
    pub mem: *mut u32,
    pub registers: [Value; 32],
    pub stack: UnsafeStack,
}

impl UnsafeContext {
    pub fn new(mem_base: *mut u32, cursor: *mut u32, stack_size: usize) -> Self {
        Self {
            mem_base,
            mem: cursor,
            registers: [Value { uint: 0 }; 32],
            stack: UnsafeStack::new(stack_size),
        }
    }
}

impl Context for UnsafeContext {
    #[inline(always)]
    fn fetch_instruction(&mut self) -> u32 {
        unsafe { *self.mem }
    }

    #[inline(always)]
    fn advance_counter(&mut self) {
        self.mem = unsafe { self.mem.add(1) };
    }

    #[inline(always)]
    fn counter(&mut self) -> *mut u32 {
        self.mem
    }

    #[inline(always)]
    fn set_counter(&mut self, counter: *mut u32) {
        self.mem = counter;
    }

    #[inline(always)]
    fn add(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint + self.registers[b].uint },
        };
    }

    #[inline(always)]
    fn add_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<16>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint + imm },
        };
    }

    #[inline(always)]
    fn addf(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            float: unsafe { self.registers[a].float + self.registers[b].float },
        };
    }

    #[inline(always)]
    fn adds(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int + self.registers[b].int },
        };
    }

    #[inline(always)]
    fn adds_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<16>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int + imm },
        };
    }

    #[inline(always)]
    fn and(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint & self.registers[b].uint },
        };
    }

    #[inline(always)]
    fn b_eq_imm(&mut self, insn: u32) {
        let a = reg(insn, 0);
        let imm = signed_immediate::<21>(insn, 5);
        if unsafe { self.registers[a].ord }.is_eq() {
            self.mem = unsafe { self.mem.offset(imm as _) };
        }
    }

    #[inline(always)]
    fn b_ge_imm(&mut self, insn: u32) {
        let a = reg(insn, 0);
        let imm = signed_immediate::<21>(insn, 5);
        if unsafe { self.registers[a].ord }.is_ge() {
            self.mem = unsafe { self.mem.offset(imm as _) };
        }
    }

    #[inline(always)]
    fn b_gt_imm(&mut self, insn: u32) {
        let a = reg(insn, 0);
        let imm = signed_immediate::<21>(insn, 5);
        if unsafe { self.registers[a].ord }.is_gt() {
            self.mem = unsafe { self.mem.offset(imm as _) };
        }
    }

    #[inline(always)]
    fn b_le_imm(&mut self, insn: u32) {
        let a = reg(insn, 0);
        let imm = signed_immediate::<21>(insn, 5);
        if unsafe { self.registers[a].ord }.is_le() {
            self.mem = unsafe { self.mem.offset(imm as _) };
        }
    }

    #[inline(always)]
    fn b_lt_imm(&mut self, insn: u32) {
        let a = reg(insn, 0);
        let imm = signed_immediate::<21>(insn, 5);
        if unsafe { self.registers[a].ord }.is_lt() {
            self.mem = unsafe { self.mem.offset(imm as _) };
        }
    }

    #[inline(always)]
    fn b_ne_imm(&mut self, insn: u32) {
        let a = reg(insn, 0);
        let imm = signed_immediate::<21>(insn, 5);
        if unsafe { self.registers[a].ord }.is_ne() {
            self.mem = unsafe { self.mem.offset(imm as _) };
        }
    }

    #[inline(always)]
    fn b_imm(&mut self, insn: u32) {
        let imm = signed_immediate::<26>(insn, 0);
        self.mem = unsafe { self.mem.offset(imm as _) };
    }

    #[inline(always)]
    fn bl_imm(&mut self, insn: u32) {
        let imm = signed_immediate::<26>(insn, 0);
        self.registers[30] = Value {
            uint: (self.mem as usize - self.mem_base as usize) as _,
        };
        self.mem = unsafe { self.mem.offset(imm as _) };
    }

    #[inline(always)]
    fn br(&mut self, insn: u32) {
        let a = reg(insn, 0);
        self.mem = (self.mem_base as usize + unsafe { self.registers[a].uint } as usize) as _;
    }

    #[inline(always)]
    fn brl(&mut self, insn: u32) {
        let a = reg(insn, 0);
        self.registers[30] = Value {
            uint: (self.mem as usize - self.mem_base as usize) as _,
        };
        self.mem = (self.mem_base as usize + unsafe { self.registers[a].uint } as usize) as _;
    }

    #[inline(always)]
    fn div(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint / self.registers[b].uint },
        };
    }

    #[inline(always)]
    fn div_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<16>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint / imm },
        };
    }

    #[inline(always)]
    fn divf(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            float: unsafe { self.registers[a].float / self.registers[b].float },
        };
    }

    #[inline(always)]
    fn divs(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int / self.registers[b].int },
        };
    }

    #[inline(always)]
    fn divs_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<16>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int / imm },
        };
    }

    #[inline(always)]
    fn halt(&mut self, _insn: u32) {
        println!("Halted");
        exit(0);
    }

    #[inline(always)]
    fn interrupt_imm(&mut self, insn: u32) {
        let _imm = immediate::<26>(insn, 0);
        todo!()
    }

    #[inline(always)]
    fn ldr(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        self.registers[dst] =
            unsafe { *((self.mem_base as usize + self.registers[a].uint as usize) as *mut _) };
    }

    #[inline(always)]
    fn ldr_imm(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let imm = signed_immediate::<21>(insn, 5);
        self.registers[dst] = unsafe { *(self.mem.offset(imm as _) as *mut _) };
    }

    #[inline(always)]
    fn mov(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        self.registers[dst] = self.registers[a];
    }

    #[inline(always)]
    fn mov_imm(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let imm = immediate::<21>(insn, 5);
        self.registers[dst] = Value { uint: imm };
    }

    #[inline(always)]
    fn movs_imm(&mut self, insn: u32) {
        let dst = reg(insn, 0);
        let imm = signed_immediate::<21>(insn, 5);
        self.registers[dst] = Value { int: imm };
    }

    #[inline(always)]
    fn mul(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint * self.registers[b].uint },
        };
    }

    #[inline(always)]
    fn mul_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<16>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint * imm },
        };
    }

    #[inline(always)]
    fn mulf(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            float: unsafe { self.registers[a].float * self.registers[b].float },
        };
    }

    #[inline(always)]
    fn muls(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int * self.registers[b].int },
        };
    }

    #[inline(always)]
    fn muls_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<16>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int * imm },
        };
    }

    #[inline(always)]
    fn nop(&mut self, _insn: u32) {}

    #[inline(always)]
    fn not(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { !self.registers[a].uint },
        };
    }

    #[inline(always)]
    fn or(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint | self.registers[b].uint },
        };
    }

    #[inline(always)]
    fn shl(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<6>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint << imm },
        };
    }

    #[inline(always)]
    fn shr(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<6>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint >> imm },
        };
    }

    #[inline(always)]
    fn shrs(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<6>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int >> imm },
        };
    }

    #[inline(always)]
    fn str(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        unsafe {
            *((self.mem_base as usize + self.registers[dst].uint as usize) as *mut _) =
                self.registers[a];
        }
    }

    #[inline(always)]
    fn str_imm(&mut self, insn: u32) {
        let a = reg(insn, 0);
        let imm = signed_immediate::<21>(insn, 5);
        unsafe { *(self.mem.offset(imm as _) as *mut _) = self.registers[a] };
    }

    #[inline(always)]
    fn sub(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint - self.registers[b].uint },
        };
    }

    #[inline(always)]
    fn sub_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = immediate::<16>(insn, 10);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint - imm },
        };
    }

    #[inline(always)]
    fn subf(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            float: unsafe { self.registers[a].float - self.registers[b].float },
        };
    }

    #[inline(always)]
    fn subs(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int - self.registers[b].int },
        };
    }

    #[inline(always)]
    fn subs_imm(&mut self, insn: u32) {
        let dst = reg(insn, 5);
        let a = reg(insn, 0);
        let imm = signed_immediate::<16>(insn, 10);
        self.registers[dst] = Value {
            int: unsafe { self.registers[a].int + imm },
        };
    }

    #[inline(always)]
    fn xor(&mut self, insn: u32) {
        let dst = reg(insn, 10);
        let a = reg(insn, 5);
        let b = reg(insn, 0);
        self.registers[dst] = Value {
            uint: unsafe { self.registers[a].uint ^ self.registers[b].uint },
        };
    }
}
