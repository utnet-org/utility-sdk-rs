//! Regular smart contract.

use unc_sdk::unc;

#[unc(contract_state)]
#[derive(Default)]
struct Incrementer {
    value: u32,
}

#[unc]
impl Incrementer {
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }
}

fn main() {}
