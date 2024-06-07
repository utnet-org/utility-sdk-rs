use unc_sdk::unc;

#[unc(contract_state)]
struct Contract {}

#[unc]
impl Contract {
    pub fn contract_source_metadata() {}
}

fn main() {}
