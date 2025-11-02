mod ambient_weather;
use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
    thread::{self, sleep},
    time::Duration,
};

use ambient_weather::error::Result;
use ambient_weather::{RealtimeClient, RealtimeEvent};

use crate::ambient_weather::DeviceDataPoint;

static NEWEST_DATA_MUTEX: LazyLock<RwLock<Option<DeviceDataPoint>>> =
    LazyLock::new(|| RwLock::new(None));
use std::sync::Arc;

use greenhouse_core::{
    smart_device_dto::{
        Type,
        config::ConfigRequestDto,
        status::{DeviceStatusDto, DeviceStatusResponseDto},
    },
    smart_device_interface::{
        config::{
            Config, Mode, TypeOptionDto, read_config_file_with_path, update_config_file_with_path,
        },
        device_builder::DeviceBuilder,
        hybrid_device::init_hybrid_router,
    },
};
use serde_derive::{Deserialize, Serialize};

const DATASOURCE_ID: &str = "7a224a14-6e07-45a3-91da-b7584a5731c1";

#[derive(Serialize, Deserialize, Clone, Default)]
struct WeatherStationConfig {
    pub application_key: String,
    pub api_key: String,
}

#[allow(clippy::needless_return)]
#[tokio::main]
async fn main() {
    // Get config path from command line arguments, default to "./config/config.json"
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./config/input_output_int_saver/config.json".to_string());

    let config: Config<WeatherStationConfig> = match read_config_file_with_path(&config_path) {
        Ok(config) => config,
        Err(_) => {
            let default_config = Config {
                mode: Mode::InputOutput,
                port: 6001,
                datasource_id: DATASOURCE_ID.to_string(),
                input_type: Some(TypeOptionDto::Number),
                output_type: Some(TypeOptionDto::Number),
                additional_config: WeatherStationConfig {
                    application_key: "".to_string(),
                    api_key: "".to_string(),
                },
                scripting_api: None,
            };

            // Create config directory if it doesn't exist
            if let Some(parent) = std::path::Path::new(&config_path).parent()
                && !parent.exists()
            {
                std::fs::create_dir_all(parent).unwrap();
            }

            // Create empty config file if it doesn't exist
            if !std::path::Path::new(&config_path).exists() {
                std::fs::write(&config_path, "{}").unwrap();
            }

            update_config_file_with_path(&default_config, &config_path).unwrap();
            panic!(
                "no config file found, created default config at {config_path} please check config file and try again"
            );
        }
    };

    let application_key = config.additional_config.application_key;
    let api_key = config.additional_config.api_key;
    thread::spawn(move || {
        init_api_scrape(application_key, api_key).unwrap();
    });

    let device_service = DeviceBuilder::new_output_device_with_config_path(
        read_handler,
        status_handler,
        config_interceptor_handler,
        &config_path,
    )
    .unwrap();
    let router = init_hybrid_router(device_service);

    let url = format!("0.0.0.0:{}", config.port);

    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    println!("using config file: {config_path}");
    axum::serve(listener, router).await.unwrap();
}

async fn read_handler(_: Arc<Config<WeatherStationConfig>>) -> Type {
    let map = NEWEST_DATA_MUTEX.read().unwrap();
    if let Some(data) = map.as_ref() {
        return Type::Object(
            data.extra
                .iter()
                .map(|(k, v)| match v {
                    serde_json::Value::Number(n) => (k.clone(), Type::Number(n.as_f64().unwrap())),
                    serde_json::Value::Bool(b) => (k.clone(), Type::Boolean(*b)),
                    _ => (k.clone(), Type::None),
                })
                .collect(),
        );
    }
    return Type::Object(HashMap::new());
}

async fn status_handler(config: Arc<Config<WeatherStationConfig>>) -> DeviceStatusResponseDto {
    DeviceStatusResponseDto {
        status: DeviceStatusDto::Online,
        datasource_id: config.datasource_id.clone(),
    }
}

async fn config_interceptor_handler(
    config: ConfigRequestDto<WeatherStationConfig>,
    old_config: Arc<Config<WeatherStationConfig>>,
) -> Config<WeatherStationConfig> {
    Config {
        mode: old_config.mode.clone(),
        port: old_config.port,
        datasource_id: old_config.datasource_id.clone(),
        input_type: old_config.input_type,
        output_type: old_config.output_type,
        additional_config: {
            WeatherStationConfig {
                application_key: config.additional_config.application_key,
                api_key: config.additional_config.api_key,
            }
        },
        scripting_api: old_config.scripting_api.clone(),
    }
}

fn cencor_qpi_key(key: String) -> String {
    if key.len() < 4 {
        return key;
    }
    return key.chars().take(4).collect::<String>()
        + "..."
        + key.chars().rev().take(4).collect::<String>().as_str();
}

fn init_api_scrape(application_key: String, api_key: String) -> Result<()> {
    let (socket, rx) =
        RealtimeClient::connect_with_base("http://localhost:8080/", &application_key)?;
    sleep(Duration::from_secs(3));

    for event in rx {
        match event {
            RealtimeEvent::Subscribed(body) => {
                println!(
                    "Subscribed: {} devices, invalid: {:?}",
                    body.devices.len(),
                    body.invalid_api_keys
                );
            }
            RealtimeEvent::Data(dp) => {
                //println!("Data: {:?}", dp);
                println!("data received");
                let mut new_data = NEWEST_DATA_MUTEX.write().unwrap();
                *new_data = Some(dp);
            }
            RealtimeEvent::Connected => {
                match RealtimeClient::subscribe(&socket, &[api_key.clone()]) {
                    Ok(_) => {
                        println!("subscribed");
                    }
                    Err(e) => {
                        println!("error subscribing: {e}");
                    }
                }
            }
        }
    }
    Ok(())
}
