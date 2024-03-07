//! Rust contract that uses conditional compilation.

use borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::unc_bindgen;

#[unc_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Incrementer {
    value: u32,
}

#[unc_bindgen]
impl Incrementer {
    #[cfg(feature = "myfeature")]
    #[init]
    pub fn new() -> Self {
        Self { value: 0 }
    }

    #[cfg(not(feature = "myfeature"))]
    #[init]
    pub fn new() -> Self {
        Self { value: 1 }
    }

    #[cfg(feature = "myfeature")]
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }

    #[cfg(not(feature = "myfeature"))]
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }
}

fn main() {}
