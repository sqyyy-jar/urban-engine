use std::alloc::{self, Layout};

use crate::context::Value;

pub struct Stack {
    pub base: *mut Value,
    pub top: *mut Value,
    pub size: usize,
}

impl Stack {
    pub fn new(size: usize) -> Self {
        let base =
            unsafe { std::alloc::alloc(alloc::Layout::from_size_align_unchecked(size * 8, 4096)) }
                as _;
        Self {
            base,
            top: base,
            size,
        }
    }

    pub fn push(&mut self, value: Value) {
        unsafe {
            *self.top = value;
            self.top = self.top.add(1);
        }
    }

    pub fn pop(&mut self) -> Value {
        unsafe {
            self.top = self.top.sub(1);
            *self.top
        }
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(
                self.base as _,
                Layout::from_size_align_unchecked(self.size * 8, 4096),
            );
        }
    }
}
