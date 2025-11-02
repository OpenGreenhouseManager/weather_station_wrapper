mod ambient_weather;
use std::{
    sync::{LazyLock, RwLock},
    thread::{self, sleep},
    time::Duration,
};

use ambient_weather::error::Result;
use ambient_weather::{RealtimeClient, RealtimeEvent};

use crate::ambient_weather::DeviceDataPoint;

static NEW_DATA_MUTEX: LazyLock<RwLock<Option<DeviceDataPoint>>> =
    LazyLock::new(|| RwLock::new(None));

fn main() -> Result<()> {
    let application_key = std::env::var("AMBIENT_APP_KEY").unwrap_or_else(|_| "app_key".into());
    let api_key = std::env::var("AMBIENT_API_KEY").unwrap_or_else(|_| "user_key".into());
    println!("0");

    init_api_scrape(application_key, api_key)?;

    println!("3");

    loop {
        {
            let new_data = NEW_DATA_MUTEX.read().unwrap();
            if let Some(dp) = &*new_data {
                println!("New data: {:?}", dp.date);
            }
        }
        sleep(Duration::from_secs(1));
    }
}

fn init_api_scrape(application_key: String, api_key: String) -> Result<()> {
    let (socket, rx) =
        RealtimeClient::connect_with_base("http://localhost:8080/", &application_key)?;
    println!("1");
    sleep(Duration::from_secs(3));

    println!("2");

    thread::spawn(move || {
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
                    let mut new_data = NEW_DATA_MUTEX.write().unwrap();
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
        panic!("thread panicked");
    });
    Ok(())
}
