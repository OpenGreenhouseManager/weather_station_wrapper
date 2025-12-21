use serde::{Deserialize, Deserializer};
use uom::si::{
    angle::degree,
    f64::ThermodynamicTemperature,
    f64::{Angle, HeatFluxDensity, Length, Pressure, Ratio, Velocity},
    heat_flux_density::watt_per_square_meter,
    length::inch,
    pressure::inch_of_mercury,
    ratio::percent,
    thermodynamic_temperature::{degree_celsius, degree_fahrenheit, degree_rankine, kelvin},
    velocity::mile_per_hour,
};

pub(crate) fn de_pressure_inhg<'de, D>(deserializer: D) -> Result<Option<Pressure>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;

    Ok(value.map(Pressure::new::<inch_of_mercury>))
}

pub(crate) fn de_thermodynamic_temperature_fahrenheit<'de, D>(
    deserializer: D,
) -> Result<Option<ThermodynamicTemperature>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;
    println!("Temp interval in raw: {:?}", value);
    println!(
        "Temp interval in fahrenheit: {:?}",
        ThermodynamicTemperature::new::<degree_fahrenheit>(value.unwrap())
    );
    Ok(value.map(ThermodynamicTemperature::new::<degree_fahrenheit>))
}

pub(crate) fn de_ratio<'de, D>(deserializer: D) -> Result<Option<Ratio>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;

    Ok(value.map(Ratio::new::<percent>))
}

pub(crate) fn de_length_inch<'de, D>(deserializer: D) -> Result<Option<Length>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;

    Ok(value.map(Length::new::<inch>))
}

pub(crate) fn de_angle_degree<'de, D>(deserializer: D) -> Result<Option<Angle>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;

    Ok(value.map(Angle::new::<degree>))
}

pub(crate) fn de_velocity_mph<'de, D>(deserializer: D) -> Result<Option<Velocity>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;

    Ok(value.map(Velocity::new::<mile_per_hour>))
}

pub(crate) fn de_power_watt<'de, D>(deserializer: D) -> Result<Option<HeatFluxDensity>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;

    Ok(value.map(HeatFluxDensity::new::<watt_per_square_meter>))
}
