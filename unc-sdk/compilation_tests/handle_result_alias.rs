use borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::unc_bindgen;

type MyResult = Result<u32, &'static str>;

#[unc_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Contract {
    value: u32,
}

#[unc_bindgen]
impl Contract {
    #[handle_result(aliased)]
    pub fn fun(&self) -> MyResult {
        Err("error")
    }
}

fn main() {}
