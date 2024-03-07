use unc_sdk::unc_bindgen;

#[unc_bindgen]
struct Contract {}

#[unc_bindgen]
impl Contract {
    pub fn contract_source_metadata() {}
}

fn main() {}
