use super::error::{Error, Result};
use super::types::{Device, DeviceDataParams, DeviceDataPoint};
use chrono::{DateTime, Utc};
use reqwest::Url;

const DEFAULT_BASE_URL: &str = "https://rt.ambientweather.net/v1";

#[derive(Clone)]
pub struct AmbientWeatherClient {
    application_key: String,
    api_key: String,
    http: reqwest::Client,
    base_url: Url,
}

impl AmbientWeatherClient {
    pub fn new(application_key: impl Into<String>, api_key: impl Into<String>) -> Result<Self> {
        let http = reqwest::Client::builder()
            .user_agent("ambient_weather-rs/0.1")
            .build()
            .map_err(|e| Error::Request(e))?;
        let base_url = Url::parse(DEFAULT_BASE_URL).map_err(|e| Error::Url(e))?;
        Ok(Self {
            application_key: application_key.into(),
            api_key: api_key.into(),
            http,
            base_url,
        })
    }

    pub fn with_base_url(
        application_key: impl Into<String>,
        api_key: impl Into<String>,
        base_url: &str,
    ) -> Result<Self> {
        let http = reqwest::Client::builder()
            .user_agent("ambient_weather-rs/0.1")
            .build()
            .map_err(|e| Error::Request(e))?;
        let base_url = Url::parse(base_url).map_err(|e| Error::Url(e))?;
        Ok(Self {
            application_key: application_key.into(),
            api_key: api_key.into(),
            http,
            base_url,
        })
    }

    pub async fn list_devices(&self) -> Result<Vec<Device>> {
        let mut url = self.base_url.join("devices").map_err(|e| Error::Url(e))?;
        url.query_pairs_mut()
            .append_pair("applicationKey", &self.application_key)
            .append_pair("apiKey", &self.api_key);

        let response = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| Error::Request(e))?;
        let response = response.error_for_status().map_err(|e| Error::Request(e))?;
        let devices: Vec<Device> = response.json().await.map_err(|e| Error::Request(e))?;
        Ok(devices)
    }

    pub async fn device_data(
        &self,
        device_mac_or_id: &str,
        params: &DeviceDataParams,
    ) -> Result<Vec<DeviceDataPoint>> {
        let mut url = self
            .base_url
            .join(&format!("devices/{device_mac_or_id}"))
            .map_err(|e| Error::Url(e))?;
        {
            let mut qp = url.query_pairs_mut();
            qp.append_pair("applicationKey", &self.application_key)
                .append_pair("apiKey", &self.api_key);
            if let Some(limit) = params.limit {
                qp.append_pair("limit", &limit.to_string());
            }
            if let Some(end) = params.end_date {
                qp.append_pair("endDate", &format_rfc3339(end));
            }
        }

        let response = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| Error::Request(e))?;
        let response = response.error_for_status().map_err(|e| Error::Request(e))?;
        // API returns an array of datapoints
        let points: Vec<DeviceDataPoint> = response.json().await.map_err(|e| Error::Request(e))?;
        Ok(points)
    }
}

fn format_rfc3339(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339()
}
