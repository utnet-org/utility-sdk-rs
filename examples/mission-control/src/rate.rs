use crate::account::*;
use crate::asset::*;
use std::collections::HashMap;
use unc_sdk::unc;

#[derive(PartialEq, Eq)]
#[unc(serializers = [json, borsh])]
pub struct Rate {
    pub credit: HashMap<Asset, Quantity>,
    pub debit: HashMap<Asset, Quantity>,
}

impl Rate {}
