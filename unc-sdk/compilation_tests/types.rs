//! Check types from unc_sdk.

use unc_sdk::unc;
use unc_sdk::collections::{LookupMap, LookupSet, TreeMap, UnorderedMap, UnorderedSet, Vector};
use unc_sdk::json_types::Base58CryptoHash;
use unc_sdk::store::{Lazy, LazyOption};
use unc_sdk::CurveType;

#[unc(contract_state)]
struct TypesContainer {
    lookup_map: LookupMap<u32, u64>,
    lookup_set: LookupSet<u32>,
    tree_map: TreeMap<u32, u64>,
    unordered_map: UnorderedMap<u32, u64>,
    unordered_set: UnorderedSet<u32>,
    vector: Vector<u32>,
    base58_crypto_hash: Base58CryptoHash,
    u64_type: unc_sdk::json_types::U64,
    base64_vec_u8: unc_sdk::json_types::Base64VecU8,
    lazy: Lazy<u64>,
    lazy_option: LazyOption<u64>,
    curve_type: CurveType,
}

fn main() {}