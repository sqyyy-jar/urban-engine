use std::collections::HashMap;

pub trait VMod<T> {
    fn load(&self, vtable: &mut HashMap<usize, fn(&mut T)>);
}
