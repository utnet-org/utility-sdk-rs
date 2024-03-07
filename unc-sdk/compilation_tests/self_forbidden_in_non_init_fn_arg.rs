//! Method signature uses Self.

use unc_sdk::unc_bindgen;
use serde::{Deserialize, Serialize};

#[unc_bindgen]
#[derive(Default, Serialize, Deserialize)]
pub struct Ident {
    value: u32,
}

#[unc_bindgen]
impl Ident {
    pub fn plain_arg(_value: Option<Self>, _value2: Self) {
        unimplemented!()
    }
}

fn main() {}
