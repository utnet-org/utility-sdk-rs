use unc_account_id::AccountIdRef;
use unc_sdk::unc;

#[unc(contract_state)]
struct Contract {}

#[unc]
impl Contract {
    pub fn anything() {}
}

#[unc]
impl Contract {
    pub fn anything_else() {}
}

fn main() {
    let ext = Contract::ext(AccountIdRef::new_or_panic("0000").into());
    ext.contract_source_metadata();
}
