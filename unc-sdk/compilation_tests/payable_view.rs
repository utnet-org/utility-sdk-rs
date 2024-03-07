//! Payable view are not valid

use borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::unc_bindgen;

#[unc_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Test {}

#[unc_bindgen]
impl Test {
    #[payable]
    pub fn pay(&self) {}
}

fn main() {}
