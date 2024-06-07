//! Regular smart contract.

use unc_sdk::unc;

#[derive(Default)]
#[unc(contract_state)]
struct Incrementer {
    value: u32,
}

#[unc]
impl Incrementer {
    #[private]
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }
}

fn main() {}
