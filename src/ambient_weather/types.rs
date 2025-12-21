use std::collections::HashMap;

use chrono::{DateTime, Utc};
use greenhouse_core::smart_device_dto::Type;
use serde::{Deserialize, Serialize};

use crate::ambient_weather::{
    barometric_sensor::BarometricSensor, indoor_sensor::IndoorSensor,
    outdoor_sensor::OutdoorSensor, probe_sensor::ProbeSensor, rain_sensor::RainSensor,
    solar_sensor::SolarSensor, wind_sensor::WindSensor,
};

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeviceDataPoint {
    pub mac_address: Option<String>,
    pub dateutc: Option<i64>,
    pub date: Option<DateTime<Utc>>,
    pub tz: Option<String>,

    #[serde(flatten)]
    pub indoor_station: IndoorSensor,
    #[serde(flatten)]
    pub outdoor_station: OutdoorSensor,
    #[serde(flatten)]
    pub generic_sensor1: ProbeSensor,
    #[serde(flatten)]
    pub rain_sensor: RainSensor,
    #[serde(flatten)]
    pub wind_sensor: WindSensor,
    #[serde(flatten)]
    pub barometric_sensor: BarometricSensor,
    #[serde(flatten)]
    pub solar_sensor: SolarSensor,
}

impl From<&DeviceDataPoint> for Type {
    fn from(data: &DeviceDataPoint) -> Self {
        Type::Object(HashMap::from([
            (String::from("indoorStation"), (&data.indoor_station).into()),
            (
                String::from("outdoorStation"),
                (&data.outdoor_station).into(),
            ),
            (
                String::from("genericSensor1"),
                (&data.generic_sensor1).into(),
            ),
            (String::from("rainSensor"), (&data.rain_sensor).into()),
            (String::from("windSensor"), (&data.wind_sensor).into()),
            (
                String::from("barometricSensor"),
                (&data.barometric_sensor).into(),
            ),
            (String::from("solarSensor"), (&data.solar_sensor).into()),
        ]))
    }
}
