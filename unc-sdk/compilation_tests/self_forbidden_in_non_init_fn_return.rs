//! Method signature uses Self.

use unc_sdk::unc;


#[derive(Default)]
#[unc(contract_state, serializers=[json])]
pub struct Ident {
    value: u32,
}

#[unc]
impl Ident {
    pub fn plain_ret() -> Self {
        unimplemented!()
    }
}

fn main() {}
