//! Smart contract with initialization function.

use unc_sdk::unc;
#[unc(contract_state)]
struct Incrementer {
    value: u32,
}

#[unc]
impl Incrementer {
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }
    #[init]
    pub fn new(starting_value: u32) -> Self {
        Self {
            value: starting_value
        }
    }
}

fn main() {}
