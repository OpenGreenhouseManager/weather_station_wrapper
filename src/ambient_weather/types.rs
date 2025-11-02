use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub mac_address: String,
    #[serde(default)]
    pub info: Option<DeviceInfo>,
    #[serde(default)]
    pub last_data: Option<DeviceDataPoint>,
    #[serde(default)]
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub elevation: Option<f64>,
    #[serde(default)]
    pub coords: Option<OuterCoords>,
    #[serde(default)]
    pub geo: Option<Geo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OuterCoords {
    #[serde(default)]
    pub coords: Option<Coords>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coords {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geo {
    #[serde(rename = "type")]
    pub geo_type: String,
    pub coordinates: [f64; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDataPoint {
    #[serde(default)]
    pub mac_address: Option<String>,
    #[serde(default)]
    pub dateutc: Option<i64>,
    #[serde(default)]
    pub date: Option<DateTime<Utc>>,
    #[serde(default)]
    pub tz: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
