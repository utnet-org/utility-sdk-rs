use std::collections::HashMap;
use std::mem::size_of;
use unc_sdk::{env, require, AccountId, Promise, UncToken};

// TODO: need a way for end users to determine how much an approval will cost.
pub fn bytes_for_approved_account_id(account_id: &AccountId) -> u64 {
    // The extra 4 bytes are coming from Borsh serialization to store the length of the string.
    account_id.as_str().len() as u64 + 4 + size_of::<u64>() as u64
}

pub fn refund_approved_account_ids_iter<'a, I>(
    account_id: AccountId,
    approved_account_ids: I,
) -> Promise
where
    I: Iterator<Item = &'a AccountId>,
{
    let storage_released: u64 = approved_account_ids.map(bytes_for_approved_account_id).sum();
    Promise::new(account_id)
        .transfer(env::storage_byte_cost().saturating_mul(storage_released.into()))
}

pub fn refund_approved_account_ids(
    account_id: AccountId,
    approved_account_ids: &HashMap<AccountId, u64>,
) -> Promise {
    refund_approved_account_ids_iter(account_id, approved_account_ids.keys())
}

pub fn refund_deposit_to_account(storage_used: u64, account_id: AccountId) {
    let required_cost = env::storage_byte_cost().saturating_mul(storage_used.into());
    let attached_deposit = env::attached_deposit();

    require!(
        required_cost <= attached_deposit,
        format!("Must attach {} attounc to cover storage", required_cost)
    );

    let refund = attached_deposit.saturating_sub(required_cost);
    if refund.as_attounc() > 1 {
        Promise::new(account_id).transfer(refund);
    }
}

/// Assumes that the precedecessor will be refunded
pub fn refund_deposit(storage_used: u64) {
    refund_deposit_to_account(storage_used, env::predecessor_account_id())
}

/// Assert that at least 1 attoUNC was attached.
pub(crate) fn assert_at_least_one_atto() {
    require!(
        env::attached_deposit() >= UncToken::from_attounc(1),
        "Requires attached deposit of at least 1 attoUNC"
    )
}
