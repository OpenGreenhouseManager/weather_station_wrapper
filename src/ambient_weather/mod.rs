//! Ambient Weather client library
//!
//! Provides access to the Ambient Weather REST API and an optional
//! Socket.IO-based realtime client (feature: `realtime`).

pub mod error;
mod realtime;
mod rest;
mod types;

pub use rest::AmbientWeatherClient;
pub use types::*;

pub use realtime::{RealtimeClient, RealtimeEvent};
