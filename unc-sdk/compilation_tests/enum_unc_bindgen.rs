//! Testing that state with enum compiles correctly

use unc_sdk::unc;

#[unc(contract_state)]
enum StateMachine {
    StateA,
    StateB,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::StateA
    }
}

#[unc]
impl StateMachine {
    pub fn swap_state(&mut self) {
        *self = match self {
            Self::StateA => Self::StateB,
            Self::StateB => Self::StateA,
        };
    }
}

fn main() {}