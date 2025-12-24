use std::collections::HashMap;

use crate::ambient_weather::deser_unit_helper::de_power_watt;
use greenhouse_core::smart_device_dto::{Measurement, Type};
use serde::Deserialize;
use uom::si::{f64::HeatFluxDensity, heat_flux_density::watt_per_square_meter};

#[derive(Debug, Clone, Deserialize, Default)]
pub(crate) struct SolarSensor {
    #[serde(rename = "uv", default)]
    pub uv_index: Option<f64>,
    #[serde(rename = "solarradiation", default, deserialize_with = "de_power_watt")]
    pub solar_radiation: Option<HeatFluxDensity>,
}

impl From<&SolarSensor> for Type {
    fn from(data: &SolarSensor) -> Self {
        let mut map = HashMap::new();
        if let Some(uv_index) = data.uv_index {
            map.insert(String::from("uvIndex"), Type::Number(uv_index));
        }
        if let Some(solar_radiation) = data.solar_radiation {
            map.insert(
                String::from("solarRadiation"),
                Type::Measurement(Measurement {
                    value: solar_radiation.get::<watt_per_square_meter>(),
                    unit: "W/m²".to_string(),
                }),
            );
        }

        Type::Object(map)
    }
}
