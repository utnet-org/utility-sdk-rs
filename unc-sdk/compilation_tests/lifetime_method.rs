//! Method signature uses lifetime.

use unc_sdk::unc;

#[unc(contract_state)]
#[derive(Default)]
struct Ident {
    value: u32,
}

#[unc]
impl Ident {
    pub fn is_ident<'a>(&self, other: &'a u32) -> Option<&'a u32> {
        if *other == self.value {
            Some(other)
        } else {
            None
        }
    }
}

fn main() {}
