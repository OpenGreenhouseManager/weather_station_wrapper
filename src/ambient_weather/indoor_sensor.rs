use std::collections::HashMap;

use crate::ambient_weather::deser_unit_helper::{
    de_ratio, de_thermodynamic_temperature_fahrenheit,
};
use greenhouse_core::smart_device_dto::Type;
use serde::Deserialize;
use uom::si::{
    f64::{Ratio, ThermodynamicTemperature},
    ratio::percent,
    thermodynamic_temperature::degree_celsius,
};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct IndoorSensor {
    #[serde(
        rename = "tempinf",
        deserialize_with = "de_thermodynamic_temperature_fahrenheit"
    )]
    pub temp: Option<ThermodynamicTemperature>,
    #[serde(rename = "humidityin", deserialize_with = "de_ratio")]
    pub humidity: Option<Ratio>,
    #[serde(
        rename = "dewPointin",
        deserialize_with = "de_thermodynamic_temperature_fahrenheit"
    )]
    pub dew_point: Option<ThermodynamicTemperature>,
    #[serde(
        rename = "feelsLikein",
        deserialize_with = "de_thermodynamic_temperature_fahrenheit"
    )]
    pub feels_like: Option<ThermodynamicTemperature>,
}

impl From<&IndoorSensor> for Type {
    fn from(data: &IndoorSensor) -> Self {
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
        Type::Object(map)
    }
}
