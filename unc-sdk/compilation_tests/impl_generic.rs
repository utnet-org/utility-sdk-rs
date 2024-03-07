//! Impl block has type parameters.

use borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::unc_bindgen;
#[allow(unused_imports)]
use std::marker::PhantomData;

#[unc_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Incrementer<T> {
    value: u32,
    data: PhantomData<T>,
}

#[unc_bindgen]
impl<'a, T: 'a + std::fmt::Display> Incrementer<T> {
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }
}

fn main() {}
