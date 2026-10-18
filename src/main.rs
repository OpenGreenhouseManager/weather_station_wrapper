mod ambient_weather;
use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
    time::Duration,
};
use tokio::time::sleep;

use ambient_weather::error::{Error, Result};
use ambient_weather::{RealtimeClient, RealtimeEvent};

use crate::ambient_weather::DeviceDataPoint;

static NEWEST_DATA_MUTEX: LazyLock<RwLock<Option<DeviceDataPoint>>> =
    LazyLock::new(|| RwLock::new(None));
static CONNECTION_STATUS: LazyLock<RwLock<DeviceStatusDto>> =
    LazyLock::new(|| RwLock::new(DeviceStatusDto::Panic));
use std::sync::Arc;

use greenhouse_core::{
    smart_device_dto::{
        Type,
        config::{ConfigRequestDto, TypeOption},
        status::{DeviceStatusDto, DeviceStatusResponseDto},
    },
    smart_device_interface::{
        config::{Config, read_config_file_with_path, update_config_file_with_path},
        device_builder::DeviceBuilder,
        hybrid_device::init_hybrid_router,
    },
};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
struct WeatherStationConfig {
    pub base_url: String,
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
                port: 6001,
                datasource_id: "7a224a14-6e07-45a3-91da-b7584a5731c1".to_string(),
                additional_config: WeatherStationConfig {
                    base_url: "http://localhost:8080/".to_string(),
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

    let base_url = config.additional_config.base_url;
    let application_key = config.additional_config.application_key;
    let api_key = config.additional_config.api_key;

    tokio::spawn(async move {
        if let Err(e) = init_api_scrape(base_url, application_key, api_key).await {
            match e {
                Error::EmitError(e) => {
                    println!("error emitting: {e}");
                }
                Error::ConnectError(e) => {
                    println!("error connecting: {e}");
                }
            }
        }
    });

    let device_service = DeviceBuilder::new_output_device_with_config_path(
        read_handler,
        status_handler,
        config_interceptor_handler,
        &config_path,
        TypeOption::Object,
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
    let map: std::sync::RwLockReadGuard<'_, Option<DeviceDataPoint>> =
        NEWEST_DATA_MUTEX.read().unwrap();
    if let Some(data) = map.as_ref() {
        return data.into();
    }
    Type::Object(HashMap::new())
}

async fn status_handler(config: Arc<Config<WeatherStationConfig>>) -> DeviceStatusResponseDto {
    DeviceStatusResponseDto {
        status: {
            let guard = CONNECTION_STATUS.read().unwrap();
            match *guard {
                DeviceStatusDto::Online => DeviceStatusDto::Online,
                DeviceStatusDto::Panic => DeviceStatusDto::Panic,
            }
        },
        datasource_id: config.datasource_id.clone(),
    }
}

async fn config_interceptor_handler(
    config: ConfigRequestDto<WeatherStationConfig>,
    old_config: Arc<Config<WeatherStationConfig>>,
) -> Config<WeatherStationConfig> {
    Config {
        port: old_config.port,
        datasource_id: old_config.datasource_id.clone(),
        additional_config: {
            WeatherStationConfig {
                base_url: config.additional_config.base_url,
                application_key: config.additional_config.application_key,
                api_key: config.additional_config.api_key,
            }
        },
        scripting_api: old_config.scripting_api.clone(),
    }
}

async fn init_api_scrape(base_url: String, application_key: String, api_key: String) -> Result<()> {
    // Exponential backoff reconnect loop
    let mut backoff_secs: u64 = 1;
    let max_backoff_secs: u64 = 30;
    loop {
        {
            let mut status = CONNECTION_STATUS.write().unwrap();
            *status = DeviceStatusDto::Panic;
        }

        match RealtimeClient::connect_with_base(&base_url, &application_key).await {
            Ok((socket, mut rx)) => {
                // Connected to transport; wait a bit, then process events
                backoff_secs = 1;
                sleep(Duration::from_secs(1)).await;

                while let Some(event) = rx.recv().await {
                    match event {
                        RealtimeEvent::Subscribed(body) => {
                            println!(
                                "Subscribed: {} devices, invalid: {:?}",
                                body.device_count, body.invalid_api_keys
                            );
                            let mut status = CONNECTION_STATUS.write().unwrap();
                            *status = DeviceStatusDto::Online;
                        }
                        RealtimeEvent::Data(dp) => {
                            println!("data received {dp:?}");
                            let mut new_data = NEWEST_DATA_MUTEX.write().unwrap();
                            *new_data = Some(dp);
                        }
                        RealtimeEvent::Connected => {
                            {
                                let mut status = CONNECTION_STATUS.write().unwrap();
                                *status = DeviceStatusDto::Online;
                            }
                            match RealtimeClient::subscribe(&socket, std::slice::from_ref(&api_key))
                                .await
                            {
                                Ok(_) => {
                                    println!("subscribed");
                                }
                                Err(e) => {
                                    println!("error subscribing: {e}");
                                }
                            }
                        }
                        RealtimeEvent::Disconnected => {
                            println!("websocket disconnected; will attempt to reconnect");
                            let mut status = CONNECTION_STATUS.write().unwrap();
                            *status = DeviceStatusDto::Panic;
                            break; // break event loop to reconnect
                        }
                    }
                }

                // Tear down the old connection so its background task stops
                let _ = socket.disconnect().await;
            }
            Err(e) => match e {
                Error::ConnectError(err) => {
                    println!("connect error: {err}");
                }
                Error::EmitError(err) => {
                    println!("emit error during connect: {err}");
                }
            },
        }

        println!("reconnecting in {backoff_secs}s...");
        sleep(Duration::from_secs(backoff_secs)).await;
        backoff_secs = (backoff_secs.saturating_mul(2)).min(max_backoff_secs);
    }
}
