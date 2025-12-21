use std::collections::HashMap;

use crate::ambient_weather::deser_unit_helper::de_pressure_inhg;
use greenhouse_core::smart_device_dto::Type;
use serde::Deserialize;
use uom::si::{f64::Pressure, pressure::hectopascal};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct BarometricSensor {
    #[serde(rename = "baromrelin", default, deserialize_with = "de_pressure_inhg")]
    pub relative_pressure: Option<Pressure>,
    #[serde(rename = "baromabsin", default, deserialize_with = "de_pressure_inhg")]
    pub absolute_pressure: Option<Pressure>,
}

impl From<&BarometricSensor> for Type {
    fn from(data: &BarometricSensor) -> Self {
        let mut map = HashMap::new();
        if let Some(relative_pressure) = data.relative_pressure {
            map.insert(
                String::from("relativePressure"),
                Type::Number(relative_pressure.get::<hectopascal>()),
            );
        }
        if let Some(absolute_pressure) = data.absolute_pressure {
            map.insert(
                String::from("absolutePressure"),
                Type::Number(absolute_pressure.get::<hectopascal>()),
            );
        }
        Type::Object(map)
    }
}
