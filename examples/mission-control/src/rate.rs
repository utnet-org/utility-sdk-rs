use crate::account::*;
use crate::asset::*;
use unc_sdk::borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "unc_sdk::serde")]
#[borsh(crate = "unc_sdk::borsh")]
pub struct Rate {
    pub credit: HashMap<Asset, Quantity>,
    pub debit: HashMap<Asset, Quantity>,
}

impl Rate {}
