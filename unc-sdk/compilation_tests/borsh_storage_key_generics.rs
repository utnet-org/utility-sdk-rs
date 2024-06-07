//! Testing BorshStorageKey macro with lifetimes and generics.

use unc_sdk::borsh::{self, BorshSerialize};
use unc_sdk::collections::LookupMap;
use unc_sdk::BorshStorageKey;
use unc_sdk::unc;

#[derive(BorshStorageKey, BorshSerialize)]
struct StorageKeyStruct<'a, T>
where
    T: ?Sized,
{
    key: &'a T,
}

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKeyEnum<'a, T>
where
    T: ?Sized,
{
    Accounts,
    SubAccounts { account_id: &'a T },
}

#[unc(contract_state)]
struct Contract {
    map1: LookupMap<u64, u64>,
    map2: LookupMap<String, String>,
}

impl Default for Contract {
    fn default() -> Self {
        let a = "test".to_string();
        Self {
            map1: LookupMap::new(StorageKeyStruct { key: "bla" }),
            map2: LookupMap::new(StorageKeyEnum::SubAccounts { account_id: &a }),
        }
    }
}

#[unc]
impl Contract {}

fn main() {}
