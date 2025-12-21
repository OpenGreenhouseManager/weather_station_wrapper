use super::error::{Error, Result};
use super::types::{Device, DeviceDataPoint};
use rust_socketio::client::Client;
use rust_socketio::{ClientBuilder, Payload, TransportType};
use serde::Deserialize;
use serde_json::json;
use std::sync::mpsc::{self, Receiver};

#[warn(clippy::large_enum_variant)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribedEventBody {
    #[serde(default)]
    pub devices: Vec<Device>,
    #[serde(default)]
    pub invalid_api_keys: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum RealtimeEvent {
    Subscribed(SubscribedEventBody),
    Data(DeviceDataPoint),
    Connected,
}

pub struct RealtimeClient {}

impl RealtimeClient {
    pub fn connect_with_base(
        base_url: &str,
        application_key: &str,
    ) -> Result<(Client, Receiver<RealtimeEvent>)> {
        let url = format!(
            "{}?api=1&applicationKey={}",
            base_url.trim_end_matches('/'),
            application_key
        );

        let (tx, rx) = mpsc::channel();

        // Clone the sender for each handler closure
        let tx_sub = tx.clone();
        let tx_connected = tx.clone();
        let tx_data = tx.clone();
        println!("url: {url}");
        let client = ClientBuilder::new(url)
            .transport_type(TransportType::Websocket)
            .on("subscribed", {
                move |payload, _| {
                    match payload {
                        Payload::Text(values) => {
                            if let Some(first) = values.into_iter().next() {
                                match serde_json::from_value::<SubscribedEventBody>(first) {
                                    Ok(body) => {
                                        let _ = tx_sub.send(RealtimeEvent::Subscribed(body));
                                    }
                                    Err(err) => {
                                        eprintln!("failed to deserialize subscribed body: {err}");
                                    }
                                }
                            } else {
                                eprintln!("subscribed event had no arguments");
                            }
                        }
                        Payload::Binary(_) => {
                            eprintln!("unexpected binary payload for 'subscribed' event");
                        }
                        #[allow(deprecated)]
                        Payload::String(s) => {
                            // Back-compat: parse legacy string payload
                            match serde_json::from_str::<SubscribedEventBody>(&s) {
                                Ok(body) => {
                                    let _ = tx_sub.send(RealtimeEvent::Subscribed(body));
                                }
                                Err(err) => eprintln!("failed to parse subscribed string: {err}"),
                            }
                        }
                    }
                }
            })
            .on("data", {
                move |payload, _| match payload {
                    Payload::Text(values) => {
                        if let Some(first) = values.into_iter().next() {
                            match serde_json::from_value::<DeviceDataPoint>(first) {
                                Ok(data) => {
                                    let _ = tx_data.send(RealtimeEvent::Data(data));
                                }
                                Err(err) => eprintln!("failed to deserialize data body: {err}"),
                            }
                        } else {
                            eprintln!("data event had no arguments");
                        }
                    }
                    Payload::Binary(_) => {
                        eprintln!("unexpected binary payload for 'data' event");
                    }
                    #[allow(deprecated)]
                    Payload::String(s) => match serde_json::from_str::<DeviceDataPoint>(&s) {
                        Ok(data) => {
                            let _ = tx_data.send(RealtimeEvent::Data(data));
                        }
                        Err(err) => eprintln!("failed to parse data string: {err}"),
                    },
                }
            })
            .on("open", {
                move |_, _| {
                    let _ = tx_connected.send(RealtimeEvent::Connected);
                }
            })
            .connect()
            .unwrap(); // .map_err(|e| Error::ConnectError(e.to_string()))?;

        Ok((client, rx))
    }

    pub fn subscribe(client: &Client, api_keys: &[String]) -> Result<()> {
        let body = json!({ "apiKeys": api_keys });
        client
            .emit("subscribe", body)
            .map_err(|e| Error::EmitError(e.to_string()))?;
        Ok(())
    }

    // pub fn unsubscribe(client: &Client, api_keys: &[String]) -> Result<()> {
    //     let body = json!({ "apiKeys": api_keys });
    //     client
    //         .emit("unsubscribe", body)
    //         .map_err(|e| Error::EmitError(e.to_string()))?;
    //     Ok(())
    // }
}
