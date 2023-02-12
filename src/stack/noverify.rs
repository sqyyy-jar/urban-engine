use std::alloc;

use crate::context::Value;

use super::Stack;

pub struct UnsafeStack {
    pub base: *mut Value,
    pub top: *mut Value,
    pub size: usize,
}

impl UnsafeStack {
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
}

impl Stack for UnsafeStack {
    #[inline(always)]
    fn push(&mut self, value: Value) {
        unsafe {
            *self.top = value;
            self.top = self.top.add(1);
        }
    }

    #[inline(always)]
    fn pop(&mut self) -> Value {
        unsafe {
            self.top = self.top.sub(1);
            *self.top
        }
    }
}

impl Drop for UnsafeStack {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(
                self.base as _,
                alloc::Layout::from_size_align_unchecked(self.size * 8, 4096),
            );
        }
    }
}
