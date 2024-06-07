//! Method with non-deserializable argument type.

use unc_sdk::unc;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, PartialOrd, Ord)]
#[unc(serializers=[borsh, json])]
enum TypeA {
    Var1,
    Var2
}

#[derive(Eq, PartialEq, Hash, PartialOrd)]
#[unc(serializers=[borsh, json])]
enum TypeB {
    Var1,
    Var2
}

#[unc(contract_state)]
#[derive(Default)]
struct Storage {
    map: HashMap<TypeA, TypeB>
}

trait MyTrait {}

#[unc]
impl Storage {
    pub fn insert(&mut self, key: TypeA, value: TypeB, t: impl MyTrait) -> Option<TypeB> {
        self.map.insert(key, value)
    }
}

fn main() {}
