use std::collections::HashMap;

use crate::ambient_weather::deser_unit_helper::{de_ratio, de_temperature_interval_fahrenheit};
use greenhouse_core::smart_device_dto::Type;
use serde::Deserialize;
use uom::si::{
    f64::{Ratio, TemperatureInterval},
    ratio::percent,
    temperature_interval::degree_celsius,
};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct OutdoorSensor {
    #[serde(
        rename = "tempf",
        default,
        deserialize_with = "de_temperature_interval_fahrenheit"
    )]
    pub temp: Option<TemperatureInterval>,
    #[serde(rename = "humidity", default, deserialize_with = "de_ratio")]
    pub humidity: Option<Ratio>,
    #[serde(
        rename = "dewPoint",
        default,
        deserialize_with = "de_temperature_interval_fahrenheit"
    )]
    pub dew_point: Option<TemperatureInterval>,
    #[serde(
        rename = "feelsLike",
        default,
        deserialize_with = "de_temperature_interval_fahrenheit"
    )]
    pub feels_like: Option<TemperatureInterval>,
    #[serde(rename = "battout", default, deserialize_with = "de_ratio")]
    pub battery: Option<Ratio>,
}

impl From<&OutdoorSensor> for Type {
    fn from(data: &OutdoorSensor) -> Self {
        let mut map = HashMap::new();
        if let Some(temp) = data.temp {
            map.insert(
                String::from("temp"),
                Type::Number(temp.get::<degree_celsius>()),
            );
        }
        if let Some(humidity) = data.humidity {
            map.insert(
                String::from("humidity"),
                Type::Number(humidity.get::<percent>()),
            );
        }
        if let Some(dew_point) = data.dew_point {
            map.insert(
                String::from("dewPoint"),
                Type::Number(dew_point.get::<degree_celsius>()),
            );
        }
        if let Some(feels_like) = data.feels_like {
            map.insert(
                String::from("feelsLike"),
                Type::Number(feels_like.get::<degree_celsius>()),
            );
        }
        if let Some(battery) = data.battery {
            map.insert(
                String::from("battery"),
                Type::Number(battery.get::<percent>()),
            );
        }
        Type::Object(map)
    }
}
