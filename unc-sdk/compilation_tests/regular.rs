//! Regular smart contract.

use unc_sdk::unc_bindgen;
use borsh::{BorshDeserialize, BorshSerialize};

#[unc_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Incrementer {
    value: u32,
}

#[unc_bindgen]
impl Incrementer {
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }
}

fn main() {}
