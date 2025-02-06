use crate::schema::*;
use reqwest::{Client as ReqwestClient, Result as ReqwestResult};
use url::Url;

const BASE_URL: &str = "https://api.mapbox.com/search/geocode/v6/";

pub struct MapboxGeocodingClient {
    client: ReqwestClient,
    _access_token: String,
    base_url: Url,
}

impl MapboxGeocodingClient {
    pub fn new(access_token: String) -> Self {
        let base_url = Url::parse(BASE_URL).expect("Failed to parse base URL");
        Self {
            client: ReqwestClient::new(),
            _access_token: access_token,
            base_url,
        }
    }

    async fn make_request<T: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        params: &T,
        method: reqwest::Method,
    ) -> ReqwestResult<R> {
        let mut url = self.base_url.join(path).expect("Failed to join URL path");

        if method == reqwest::Method::GET {
            let query =
                serde_urlencoded::to_string(params).expect("Failed to serialize parameters");
            url.set_query(Some(&query));
        }

        let mut request = self.client.request(method.clone(), url);
        if method == reqwest::Method::POST {
            request = request.json(params);
        }

        request.send().await?.error_for_status()?.json::<R>().await
    }

    pub async fn forward_geocode(
        &self,
        params: ForwardGeocodingParams,
    ) -> ReqwestResult<GeocodingResponse> {
        self.make_request("forward", &params, reqwest::Method::GET)
            .await
    }

    pub async fn structured_forward_geocode(
        &self,
        params: StructuredGeocodingParams,
    ) -> ReqwestResult<GeocodingResponse> {
        self.make_request("forward", &params, reqwest::Method::GET)
            .await
    }

    pub async fn reverse_geocode(
        &self,
        params: ReverseGeocodingParams,
    ) -> ReqwestResult<GeocodingResponse> {
        self.make_request("reverse", &params, reqwest::Method::GET)
            .await
    }

    pub async fn batch_geocode(
        &self,
        queries: Vec<ForwardGeocodingParams>,
    ) -> ReqwestResult<GeocodingResponse> {
        self.make_request("batch", &queries, reqwest::Method::POST)
            .await
    }
}
