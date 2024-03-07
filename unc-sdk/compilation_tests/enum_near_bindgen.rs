//! Testing that state with enum compiles correctly

use borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::unc_bindgen;

#[unc_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
enum StateMachine {
    StateA,
    StateB,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::StateA
    }
}

#[unc_bindgen]
impl StateMachine {
    pub fn swap_state(&mut self) {
        *self = match self {
            Self::StateA => Self::StateB,
            Self::StateB => Self::StateA,
        };
    }
}

fn main() {}