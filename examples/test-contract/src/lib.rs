use unc_sdk::borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::{env, unc_bindgen};

#[unc_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[borsh(crate = "unc_sdk::borsh")]
pub struct TestContract {}

impl Default for TestContract {
    fn default() -> Self {
        Self {}
    }
}

#[unc_bindgen]
impl TestContract {
    #[init]
    pub fn new() -> Self {
        Self {}
    }

    #[init(ignore_state)]
    pub fn migrate_state() -> Self {
        #[derive(BorshDeserialize)]
        #[borsh(crate = "unc_sdk::borsh")]
        struct OldContract {
            // ...
        }

        let _old_contract: OldContract = env::state_read().expect("Old state doesn't exist");

        Self {}
    }

    pub fn test_panic_macro(&mut self) {
        panic!("PANIC!");
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "PANIC!")]
    fn test_panic() {
        let mut contract = TestContract::new();
        contract.test_panic_macro();
    }
}
