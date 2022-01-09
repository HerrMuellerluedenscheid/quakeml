use chrono::{DateTime, Utc};
use serde::Deserialize;

pub type ResourceReference = String;
pub type GroundTruthLevel = String;
pub type OriginUncertaintyDescription = String;

#[derive(Debug, Deserialize, PartialEq)]
pub struct RealQuantity {
    pub value: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TimeQuantity {
    pub value: DateTime<Utc>,
    pub uncertainty: Option<f64>,
    pub lower_uncertainty: Option<f64>,
    pub upper_uncertainty: Option<f64>,
    pub confidence_level: Option<f64>,
}
