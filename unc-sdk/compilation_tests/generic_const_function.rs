//! Functions can't use const generics.

use unc_sdk::unc;


#[derive(Default)]
#[unc(contract_state)]
struct Ident {
    value: u32,
}

#[unc]
impl Ident {
    pub fn is_ident_const<const N: usize>(&self, val: [u32; N]) -> [u32; N] {
        val
    }
}

fn main() {}
