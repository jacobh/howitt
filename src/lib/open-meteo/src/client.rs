use crate::schema::*;
use reqwest::{Client as ReqwestClient, Result as ReqwestResult};
use url::Url;

const BASE_URL: &str = "https://archive-api.open-meteo.com";

pub struct OpenMeteoHistoryClient {
    client: ReqwestClient,
    base_url: Url,
}

impl OpenMeteoHistoryClient {
    pub fn new() -> Self {
        let base_url = Url::parse(BASE_URL).expect("Failed to parse base URL");
        Self {
            client: ReqwestClient::new(),
            base_url,
        }
    }

    async fn make_request<T: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        params: &T,
    ) -> ReqwestResult<R> {
        let mut url = self.base_url.join(path).expect("Failed to join URL path");

        let query = serde_urlencoded::to_string(params).expect("Failed to serialize parameters");
        url.set_query(Some(&query));

        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<R>()
            .await
    }

    pub async fn get_historical_weather(
        &self,
        params: HistoricalWeatherParams,
    ) -> ReqwestResult<HistoricalWeatherResponse> {
        self.make_request("/v1/archive", &params).await
    }
}

impl Default for OpenMeteoHistoryClient {
    fn default() -> Self {
        Self::new()
    }
}
