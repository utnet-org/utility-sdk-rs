use unc_sdk::unc;

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
)]
#[unc(serializers = [json, borsh])]
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
)]
#[unc(serializers = [json, borsh])]
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
)]
#[unc(serializers = [json, borsh])]
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
    PartialOrd,
    Ord,
)]
#[unc(serializers = [json, borsh])]
pub enum Exchange {
    MissionTimeWithResource,
    MissionTimeWithTrust,
}
