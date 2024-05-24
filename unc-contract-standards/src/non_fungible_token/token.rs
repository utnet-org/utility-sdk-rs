use crate::non_fungible_token::metadata::TokenMetadata;
use std::collections::HashMap;
use unc_sdk::serde::{Deserialize, Serialize};
use unc_sdk::{AccountId, UncSchema};
/// Note that token IDs for NFTs are strings on UNC. It's still fine to use autoincrementing numbers as unique IDs if desired, but they should be stringified. This is to make IDs more future-proof as chain-agnostic conventions and standards arise, and allows for more flexibility with considerations like bridging NFTs across chains, etc.
pub type TokenId = String;

/// In this implementation, the Token struct takes two extensions standards (metadata and approval) as optional fields, as they are frequently used in modern NFTs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, UncSchema)]
#[serde(crate = "unc_sdk::serde")]
pub struct Token {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: Option<TokenMetadata>,
    pub approved_account_ids: Option<HashMap<AccountId, u64>>,
}
