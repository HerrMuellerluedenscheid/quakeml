use serde::Deserialize;
use chrono::{DateTime, Utc};

pub type ResourceReference = String;

#[derive(Debug, Deserialize, PartialEq)]
pub struct RealQuantity {
    pub value: f64
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TimeQuantity {
    pub value: DateTime<Utc>
}
