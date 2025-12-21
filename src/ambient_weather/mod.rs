//! Ambient Weather client library
//!
//! Provides access to the Ambient Weather REST API and an optional
//! Socket.IO-based realtime client (feature: `realtime`).

pub mod barometric_sensor;
pub mod deser_unit_helper;
pub mod error;
pub mod indoor_sensor;
pub mod outdoor_sensor;
pub mod probe_sensor;
pub mod rain_sensor;
pub mod realtime;
pub mod solar_sensor;
mod types;
pub mod wind_sensor;

pub use types::*;

pub use realtime::{RealtimeClient, RealtimeEvent};
