//! Payable view are not valid

use unc_sdk::unc;


#[derive(Default)]
#[unc(contract_state)]
struct Test {}

#[unc]
impl Test {
    #[payable]
    pub fn pay(&self) {}
}

fn main() {}
