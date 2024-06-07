//! Functions can't use generics.

use unc_sdk::unc;


#[derive(Default)]
#[unc(contract_state)]
struct Ident {
    value: u32,
}

#[unc]
impl Ident {
    pub fn is_ident<T>(&self, val: T) -> T {
        val
    }
}

fn main() {}
