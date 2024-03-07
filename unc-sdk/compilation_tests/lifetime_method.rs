//! Method signature uses lifetime.

use unc_sdk::unc_bindgen;
use borsh::{BorshDeserialize, BorshSerialize};

#[unc_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Ident {
    value: u32,
}

#[unc_bindgen]
impl Ident {
    pub fn is_ident<'a>(&self, other: &'a u32) -> Option<&'a u32> {
        if *other == self.value {
            Some(other)
        } else {
            None
        }
    }
}

fn main() {}
