use std::collections::HashMap;

use crate::ambient_weather::deser_unit_helper::de_length_inch;
use greenhouse_core::smart_device_dto::Type;
use serde::Deserialize;
use uom::si::{f64::Length, length::millimeter};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RainSensor {
    #[serde(rename = "eventrainin", default, deserialize_with = "de_length_inch")]
    pub event_rain: Option<Length>,
    #[serde(rename = "dailyrainin", default, deserialize_with = "de_length_inch")]
    pub daily_rain: Option<Length>,
    #[serde(rename = "hourlyrainin", default, deserialize_with = "de_length_inch")]
    pub hourly_rain_rate: Option<Length>,
    #[serde(rename = "weeklyrainin", default, deserialize_with = "de_length_inch")]
    pub weekly_rain: Option<Length>,
    #[serde(rename = "monthlyrainin", default, deserialize_with = "de_length_inch")]
    pub monthly_rain: Option<Length>,
    #[serde(rename = "yearlyrainin", default, deserialize_with = "de_length_inch")]
    pub yearly_rain: Option<Length>,
    #[serde(rename = "totalrainin", default, deserialize_with = "de_length_inch")]
    pub total_rain: Option<Length>,
}

impl From<&RainSensor> for Type {
    fn from(data: &RainSensor) -> Self {
        let mut map = HashMap::new();
        if let Some(event_rain) = data.event_rain {
            map.insert(
                String::from("eventRain"),
                Type::Number(event_rain.get::<millimeter>()),
            );
        }
        if let Some(daily_rain) = data.daily_rain {
            map.insert(
                String::from("dailyRain"),
                Type::Number(daily_rain.get::<millimeter>()),
            );
        }
        if let Some(hourly_rain_rate) = data.hourly_rain_rate {
            map.insert(
                String::from("hourlyRainRate"),
                Type::Number(hourly_rain_rate.get::<millimeter>()),
            );
        }
        if let Some(weekly_rain) = data.weekly_rain {
            map.insert(
                String::from("weeklyRain"),
                Type::Number(weekly_rain.get::<millimeter>()),
            );
        }
        if let Some(monthly_rain) = data.monthly_rain {
            map.insert(
                String::from("monthlyRain"),
                Type::Number(monthly_rain.get::<millimeter>()),
            );
        }
        if let Some(yearly_rain) = data.yearly_rain {
            map.insert(
                String::from("yearlyRain"),
                Type::Number(yearly_rain.get::<millimeter>()),
            );
        }
        if let Some(total_rain) = data.total_rain {
            map.insert(
                String::from("totalRain"),
                Type::Number(total_rain.get::<millimeter>()),
            );
        }
        Type::Object(map)
    }
}
