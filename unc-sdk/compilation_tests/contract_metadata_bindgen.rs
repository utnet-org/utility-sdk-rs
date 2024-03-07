use unc_account_id::AccountIdRef;
use unc_sdk::unc_bindgen;

#[unc_bindgen]
struct Contract {}

#[unc_bindgen]
impl Contract {
    pub fn anything() {}
}

#[unc_bindgen]
impl Contract {
    pub fn anything_else() {}
}

fn main() {
    let ext = Contract::ext(AccountIdRef::new_or_panic("0000").into());
    ext.contract_source_metadata();
}
