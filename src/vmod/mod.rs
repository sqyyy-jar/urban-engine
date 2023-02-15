use std::collections::HashMap;

use crate::context::Context;

pub mod util;

pub trait VMod<T: Context> {
    fn load(&self, vtable: &mut HashMap<usize, fn(&mut T)>);
}
