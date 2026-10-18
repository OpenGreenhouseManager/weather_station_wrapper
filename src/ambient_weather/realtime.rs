use super::error::{Error, Result};
use super::types::DeviceDataPoint;
use futures_util::FutureExt;
use futures_util::future::BoxFuture;
use rust_socketio::asynchronous::{Client, ClientBuilder};
use rust_socketio::{Payload, TransportType};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::json;
use tokio::sync::mpsc::{self, UnboundedReceiver};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribedEventBody {
    #[serde(default)]
    pub device_count: usize,
    #[serde(default)]
    pub invalid_api_keys: Vec<String>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum RealtimeEvent {
    Subscribed(SubscribedEventBody),
    Data(DeviceDataPoint),
    Connected,
    Disconnected,
}

pub struct RealtimeClient {}

impl RealtimeClient {
    pub async fn connect_with_base(
        base_url: &str,
        application_key: &str,
    ) -> Result<(Client, UnboundedReceiver<RealtimeEvent>)> {
        let url = format!(
            "{}?api=1&applicationKey={}",
            base_url.trim_end_matches('/'),
            application_key
        );

        let (tx, rx) = mpsc::unbounded_channel();

        // Clone the sender for each handler closure
        let tx_sub = tx.clone();
        let tx_connected = tx.clone();
        let tx_data = tx.clone();
        let tx_close = tx.clone();
        let tx_error = tx;
        println!("url: {url}");
        let client = ClientBuilder::new(url)
            .transport_type(TransportType::Websocket)
            // Reconnects are handled by the caller's backoff loop, so the
            // library must not keep its own reconnect task running.
            .reconnect(false)
            .on("subscribed", move |payload, _| {
                if let Some(body) = parse_payload::<SubscribedEventBody>(payload, "subscribed") {
                    let _ = tx_sub.send(RealtimeEvent::Subscribed(body));
                }
                done()
            })
            .on("data", move |payload, _| {
                if let Some(data) = parse_payload::<DeviceDataPoint>(payload, "data") {
                    let _ = tx_data.send(RealtimeEvent::Data(data));
                }
                done()
            })
            .on("open", move |_, _| {
                let _ = tx_connected.send(RealtimeEvent::Connected);
                done()
            })
            .on("close", move |_, _| {
                let _ = tx_close.send(RealtimeEvent::Disconnected);
                done()
            })
            .on("error", move |payload, _| {
                eprintln!("socket error: {payload:?}");
                let _ = tx_error.send(RealtimeEvent::Disconnected);
                done()
            })
            .connect()
            .await
            .map_err(|e| Error::ConnectError(e.to_string()))?;

        Ok((client, rx))
    }

    pub async fn subscribe(client: &Client, api_keys: &[String]) -> Result<()> {
        let body = json!({ "apiKeys": api_keys });
        client
            .emit("subscribe", body)
            .await
            .map_err(|e| Error::EmitError(e.to_string()))?;
        Ok(())
    }

    // pub async fn unsubscribe(client: &Client, api_keys: &[String]) -> Result<()> {
    //     let body = json!({ "apiKeys": api_keys });
    //     client
    //         .emit("unsubscribe", body)
    //         .await
    //         .map_err(|e| Error::EmitError(e.to_string()))?;
    //     Ok(())
    // }
}

fn done() -> BoxFuture<'static, ()> {
    async {}.boxed()
}

fn parse_payload<T: DeserializeOwned>(payload: Payload, event: &str) -> Option<T> {
    match payload {
        Payload::Text(values) => {
            let Some(first) = values.into_iter().next() else {
                eprintln!("{event} event had no arguments");
                return None;
            };
            serde_json::from_value(first)
                .map_err(|err| eprintln!("failed to deserialize {event} body: {err}"))
                .ok()
        }
        Payload::Binary(_) => {
            eprintln!("unexpected binary payload for '{event}' event");
            None
        }
        // Back-compat: parse legacy string payload
        #[allow(deprecated)]
        Payload::String(s) => serde_json::from_str(&s)
            .map_err(|err| eprintln!("failed to parse {event} string: {err}"))
            .ok(),
    }
}
