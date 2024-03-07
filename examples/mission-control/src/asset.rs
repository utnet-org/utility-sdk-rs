use unc_sdk::UncSchema;
use unc_sdk::borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::serde::{Deserialize, Serialize};

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    BorshDeserialize,
    BorshSerialize,
    UncSchema,
)]
#[serde(crate = "unc_sdk::serde")]
#[borsh(crate = "unc_sdk::borsh")]
pub enum Resource {
    Battery,
    RgbSensor,
    ThermalSensor,
    PoseEstimation,
}

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    BorshDeserialize,
    BorshSerialize,
    UncSchema,
)]
#[serde(crate = "unc_sdk::serde")]
#[borsh(crate = "unc_sdk::borsh")]
pub enum Reward {
    Score,
    Token,
    Prediction,
    Currency,
    Policy,
}

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    BorshDeserialize,
    BorshSerialize,
    UncSchema,
)]
#[serde(crate = "unc_sdk::serde")]
#[borsh(crate = "unc_sdk::borsh")]
pub enum Asset {
    Resource(Resource),
    Reward(Reward),
    MissionTime,
    Trust,
}

#[derive(
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    PartialOrd,
    Ord,
    BorshDeserialize,
    BorshSerialize,
    UncSchema
)]
#[serde(crate = "unc_sdk::serde")]
#[borsh(crate = "unc_sdk::borsh")]
pub enum Exchange {
    MissionTimeWithResource,
    MissionTimeWithTrust,
}
