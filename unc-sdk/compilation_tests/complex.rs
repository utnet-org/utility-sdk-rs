//! Complex smart contract.

use unc_sdk::unc;
use std::collections::HashMap;

#[derive(
    Eq, PartialEq, Hash, PartialOrd, Ord,
)]
#[unc(serializers=[borsh, json])]
pub enum TypeA {
    Var1,
    Var2,
}

#[derive(
    Eq, PartialEq, Hash, PartialOrd,
)]
#[unc(serializers=[borsh, json])]
pub enum TypeB {
    Var1,
    Var2,
}

#[unc(contract_state)]
#[derive(Default)]
struct Storage {
    map: HashMap<TypeA, TypeB>,
}

#[unc]
impl Storage {
    pub fn insert(&mut self, key: TypeA, value: TypeB) -> Option<TypeB> {
        self.map.insert(key, value)
    }
}

fn main() {}
