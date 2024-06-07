//! Even though it might feel unintuitive, a method can be both private and init.
//! See: https://github.com/unc/unc-sdk-rs/issues/1040#issuecomment-1687126452

use unc_sdk::unc;

#[unc(contract_state)]
struct Incrementer {
    value: u32,
}

#[unc]
impl Incrementer {
    #[private]
    #[init]
    pub fn new(starting_value: u32) -> Self {
        Self { value: starting_value }
    }
}

fn main() {}
