//! Smart contract that implements trait.

use unc_sdk::unc_bindgen;
use borsh::{BorshDeserialize, BorshSerialize};

#[unc_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Incrementer {
    value: u32,
}

pub trait Zeroable {
    fn set_to_zero(&mut self);
}

#[unc_bindgen]
impl Incrementer {
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }
}

#[unc_bindgen]
impl Zeroable for Incrementer {
    fn set_to_zero(&mut self) {
        self.value = 0;
    }
}

fn main() {}
