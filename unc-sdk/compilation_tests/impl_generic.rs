//! Impl block has type parameters.

use unc_sdk::unc;
#[allow(unused_imports)]
use std::marker::PhantomData;

#[unc(contract_state)]
#[derive(Default)]
struct Incrementer<T> {
    value: u32,
    data: PhantomData<T>,
}

#[unc]
impl<'a, T: 'a + std::fmt::Display> Incrementer<T> {
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }
}

fn main() {}
