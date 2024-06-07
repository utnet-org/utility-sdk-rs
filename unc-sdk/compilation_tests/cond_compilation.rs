//! Rust contract that uses conditional compilation.

use unc_sdk::unc;

#[unc(contract_state)]
#[derive(Default)]
struct Incrementer {
    value: u32,
}

#[unc]
impl Incrementer {
    #[cfg(feature = "myfeature")]
    #[init]
    pub fn new() -> Self {
        Self { value: 0 }
    }

    #[cfg(not(feature = "myfeature"))]
    #[init]
    pub fn new() -> Self {
        Self { value: 1 }
    }

    #[cfg(feature = "myfeature")]
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }

    #[cfg(not(feature = "myfeature"))]
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }
}

fn main() {}
