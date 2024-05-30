// We want to enable all clippy lints, but some of them generate false positives.
#![allow(clippy::missing_const_for_fn, clippy::redundant_pub_crate)]

pub mod fungible_token;
pub mod non_fungible_token;
pub mod storage_management;
/// This upgrade standard is a use case where a staging area exists for a WASM
/// blob, allowing it to be stored for a period of time before deployed.
#[deprecated(
    since = "1.1.0",
    note = "This was removed because there is no standard (UIP) for upgradable contracts."
)]
pub mod upgrade;

pub(crate) mod event;

pub mod contract_metadata;
