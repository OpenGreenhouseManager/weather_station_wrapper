use std::collections::HashMap;

use crate::ambient_weather::deser_unit_helper::{de_angle_degree, de_velocity_mph};
use greenhouse_core::smart_device_dto::Type;
use serde::Deserialize;
use uom::si::{
    angle::degree,
    f64::{Angle, Velocity},
    velocity::meter_per_second,
};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct WindSensor {
    #[serde(rename = "winddir", default, deserialize_with = "de_angle_degree")]
    pub wind_direction: Option<Angle>,
    #[serde(rename = "maxdailygust", default, deserialize_with = "de_velocity_mph")]
    pub wind_speed_daily_max: Option<Velocity>,
    #[serde(rename = "windgustmph", default, deserialize_with = "de_velocity_mph")]
    pub wind_speed_10m_max: Option<Velocity>,
    #[serde(rename = "windspeedmph", default, deserialize_with = "de_velocity_mph")]
    pub wind_speed_mph: Option<Velocity>,
}

impl From<&WindSensor> for Type {
    fn from(data: &WindSensor) -> Self {
        let mut map = HashMap::new();
        if let Some(wind_direction) = data.wind_direction {
            map.insert(
                String::from("windDirection"),
                Type::Number(wind_direction.get::<degree>()),
            );
        }
        if let Some(wind_speed_daily_max) = data.wind_speed_daily_max {
            map.insert(
                String::from("windSpeedDailyMax"),
                Type::Number(wind_speed_daily_max.get::<meter_per_second>()),
            );
        }
        if let Some(wind_speed_10m_max) = data.wind_speed_10m_max {
            map.insert(
                String::from("windSpeed10mMax"),
                Type::Number(wind_speed_10m_max.get::<meter_per_second>()),
            );
        }
        if let Some(wind_speed_mph) = data.wind_speed_mph {
            map.insert(
                String::from("windSpeedMph"),
                Type::Number(wind_speed_mph.get::<meter_per_second>()),
            );
        }
        Type::Object(map)
    }
}
