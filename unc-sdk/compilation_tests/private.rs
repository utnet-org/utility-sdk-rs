//! Regular smart contract.

use borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::unc_bindgen;

#[unc_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Incrementer {
    value: u32,
}

#[unc_bindgen]
impl Incrementer {
    #[private]
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }
}

fn main() {}
