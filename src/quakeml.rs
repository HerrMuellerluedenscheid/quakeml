use std::fmt;
use std::fmt::Formatter;
use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use quick_xml::de::from_str;
use serde::Deserialize;

include!("base_types.rs");

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct CreationInfo {
    agency_id: Option<String>,
    agency_uri: Option<String>, // should be serialized to http::Uri instead
    creation_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct OriginQuality {
    used_station_count: Option<i32>,
    used_phase_count: Option<i32>,
    standard_error: Option<f64>,
    azimuthal_gap: Option<f64>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct OriginUncertainty {
    horizontal_uncertainty: f64,
    preferred_description: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Origin {
    #[serde(rename = "publicID")]
    public_id: ResourceReference,
    time: TimeQuantity,
    longitude: RealQuantity,
    latitude: RealQuantity,
    depth: RealQuantity,
    origin_uncertainty: Option<OriginUncertainty>,
    quality: Option<OriginQuality>,
    evaluation_mode: Option<String>,
    creation_info: Option<CreationInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Magnitude {
    mag: RealQuantity,
    creation_time: Option<DateTime<Utc>>,
    #[serde(rename = "publicID")]
    public_id: ResourceReference,

    #[serde(rename = "originID")]
    origin_id: Option<ResourceReference>,

    #[serde(rename = "methodID")]
    method_id: Option<ResourceReference>,

    #[serde(rename = "type")]
    _type: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct EventDescription {
    text: String,

    #[serde(rename = "type")]
    _type: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Event {
    origin: Vec<Origin>,

    #[serde(rename = "publicID")]
    public_id: ResourceReference,

    #[serde(rename = "magnitude")]
    magnitudes: Vec<Magnitude>,
    description: Vec<EventDescription>,

    #[serde(rename = "preferredOriginID")]
    preferred_origin_id: Option<ResourceReference>,
    preferred_magnitude_id: Option<ResourceReference>,
}

impl Event {
    fn preferred_origin(&self) -> Option<&Origin> {
        // Get the preferred origin
        // Returns the first origin if there is only one origin defined.
        // Otherwise, returns the origin matching the defined preferred_origin_id.

        if self.origin.len() == 1 {
            return Some(&self.origin[0]);
        } else {
            let preferred_origin = self
                .origin
                .iter()
                .find(|&origin| origin.public_id == *self.preferred_origin_id.as_ref().unwrap())
                .expect("Didn't find an origin with preferred_origin_id");
            Some(&preferred_origin)
        }
    }

    fn preferred_magnitude(&self) -> Option<f64> {
        // Get the preferred magnitude as f64
        // Returns the first magnitude if there is only one magnitude defined.
        // Otherwise, returns the magnitude matching the defined preferred_magnitude_id.
        if self.magnitudes.len() == 0 {
            return None;
        }

        let mut preferred_magnitude = &self.magnitudes[0];

        if self.magnitudes.len() >= 2 {
            preferred_magnitude = self
                .magnitudes
                .iter()
                .find(|&mag| mag.public_id == *self.preferred_magnitude_id.as_ref().unwrap())
                .expect("Didn't find a magnitude with preferred_magnitude_id");
        }

        Some(preferred_magnitude.mag.value)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct EventParameters {
    #[serde(rename = "event")]
    events: Vec<Event>,
    creation_info: CreationInfo,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct QuakeML {
    event_parameters: EventParameters,
}

impl QuakeML {
    fn min_magnitude(&self) -> Option<f64> {
        let min_mag_event = &self.event_parameters.events.iter().min_by(|a, b| {
            a.preferred_magnitude()
                .unwrap()
                .partial_cmp(&b.preferred_magnitude().unwrap())
                .unwrap()
        });
        return min_mag_event.unwrap().preferred_magnitude();
    }
    fn max_magnitude(&self) -> Option<f64> {
        let max_mag_event = &self.event_parameters.events.iter().max_by(|a, b| {
            a.preferred_magnitude()
                .unwrap()
                .partial_cmp(&b.preferred_magnitude().unwrap())
                .unwrap()
        });
        return max_mag_event.unwrap().preferred_magnitude();
    }
}

impl fmt::Display for QuakeML {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Events in catalog: {}\n Max magnitude: {}\n Min magnitude: {}",
            self.event_parameters.events.len(),
            self.max_magnitude().unwrap(),
            self.min_magnitude().unwrap(),
        )
    }
}

pub fn deserialize_quakeml(data: String) -> QuakeML {
    let quakeml: QuakeML = from_str(&*data).expect("something went wrong");
    quakeml
}

pub fn read_quakeml(filename: &PathBuf) -> QuakeML {
    let data =
        fs::read_to_string(filename).expect("Failed to read quakeml into string");
    deserialize_quakeml(data)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::read_quakeml;

    #[test]
    fn catalog_attributes() {
        let mut data_sample = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_sample.push("resources/sample.quakeml");
        let _catalog = read_quakeml(&data_sample);
        assert_eq!(
            _catalog.event_parameters.events[0].public_id,
            "quakeml:earthquake.usgs.gov/fdsnws/event/1/query?eventid=ci14517572&format=quakeml"
        );
    }
}
