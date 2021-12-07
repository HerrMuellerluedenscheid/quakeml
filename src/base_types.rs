
#[derive(Debug, Deserialize, PartialEq)]
struct RealQuantity {
    value: f64
}

#[derive(Debug, Deserialize, PartialEq)]
struct TimeQuantity {
    value: DateTime<Utc>
}
