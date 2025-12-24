use std::collections::HashMap;

use crate::ambient_weather::deser_unit_helper::de_pressure_inhg;
use greenhouse_core::smart_device_dto::{Measurement, Type};
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
                Type::Measurement(Measurement {
                    value: relative_pressure.get::<hectopascal>(),
                    unit: "hPa".to_string(),
                }),
            );
        }
        if let Some(absolute_pressure) = data.absolute_pressure {
            map.insert(
                String::from("absolutePressure"),
                Type::Measurement(Measurement {
                    value: absolute_pressure.get::<hectopascal>(),
                    unit: "hPa".to_string(),
                }),
            );
        }
        Type::Object(map)
    }
}
