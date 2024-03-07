//! Functions can't use generics.

use borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::unc_bindgen;

#[unc_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Ident {
    value: u32,
}

#[unc_bindgen]
impl Ident {
    pub fn is_ident<T>(&self, val: T) -> T {
        val
    }
}

fn main() {}
