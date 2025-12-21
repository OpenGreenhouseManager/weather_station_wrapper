use serde::{Deserialize, Deserializer};
use uom::si::{
    angle::degree,
    f64::{Angle, HeatFluxDensity, Length, Pressure, Ratio, TemperatureInterval, Velocity},
    heat_flux_density::watt_per_square_meter,
    length::inch,
    pressure::inch_of_mercury,
    ratio::percent,
    temperature_interval::degree_fahrenheit,
    velocity::mile_per_hour,
};

pub(crate) fn de_pressure_inhg<'de, D>(deserializer: D) -> Result<Option<Pressure>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;

    Ok(value.map(|inhg| Pressure::new::<inch_of_mercury>(inhg)))
}

pub(crate) fn de_temperature_interval_fahrenheit<'de, D>(
    deserializer: D,
) -> Result<Option<TemperatureInterval>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;

    Ok(value.map(|fahrenheit| TemperatureInterval::new::<degree_fahrenheit>(fahrenheit)))
}

pub(crate) fn de_ratio<'de, D>(deserializer: D) -> Result<Option<Ratio>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;

    Ok(value.map(|ratio| Ratio::new::<percent>(ratio)))
}

pub(crate) fn de_length_inch<'de, D>(deserializer: D) -> Result<Option<Length>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;

    Ok(value.map(|length| Length::new::<inch>(length)))
}

pub(crate) fn de_angle_degree<'de, D>(deserializer: D) -> Result<Option<Angle>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;

    Ok(value.map(|angle| Angle::new::<degree>(angle)))
}

pub(crate) fn de_velocity_mph<'de, D>(deserializer: D) -> Result<Option<Velocity>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;

    Ok(value.map(|velocity| Velocity::new::<mile_per_hour>(velocity)))
}

pub(crate) fn de_power_watt<'de, D>(deserializer: D) -> Result<Option<HeatFluxDensity>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;

    Ok(value
        .map(|heat_flux_density| HeatFluxDensity::new::<watt_per_square_meter>(heat_flux_density)))
}
