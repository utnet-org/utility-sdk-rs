//! Testing BorshStorageKey macro with lifetimes and generics.

use unc_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use unc_sdk::collections::LookupMap;
use unc_sdk::{unc_bindgen, BorshStorageKey};

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

#[unc_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
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

#[unc_bindgen]
impl Contract {}

fn main() {}
