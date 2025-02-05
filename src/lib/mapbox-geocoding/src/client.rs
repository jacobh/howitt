use crate::schema::*;
use reqwest::{Client as ReqwestClient, Result as ReqwestResult};
use url::Url;

const BASE_URL: &str = "https://api.mapbox.com/search/geocode/v6/";

pub struct MapboxGeocodingClient {
    client: ReqwestClient,
    access_token: String,
    base_url: Url,
}

impl MapboxGeocodingClient {
    pub fn new(access_token: String) -> Self {
        let base_url = Url::parse(BASE_URL).expect("Failed to parse base URL");
        Self {
            client: ReqwestClient::new(),
            access_token,
            base_url,
        }
    }

    /// Forward geocoding with search text input
    pub async fn forward_geocode(
        &self,
        params: ForwardGeocodingParams,
    ) -> ReqwestResult<GeocodingResponse> {
        let mut url = self
            .base_url
            .join("forward")
            .expect("Failed to join URL path");

        let query = serde_urlencoded::to_string(params).expect("Failed to serialize parameters");
        url.set_query(Some(&query));

        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<GeocodingResponse>()
            .await
    }

    /// Forward geocoding with structured input
    pub async fn structured_forward_geocode(
        &self,
        params: StructuredGeocodingParams,
    ) -> ReqwestResult<GeocodingResponse> {
        let mut url = self
            .base_url
            .join("forward")
            .expect("Failed to join URL path");

        let query = serde_urlencoded::to_string(&params).expect("Failed to serialize parameters");
        url.set_query(Some(&query));

        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<GeocodingResponse>()
            .await
    }

    /// Reverse geocoding
    pub async fn reverse_geocode(
        &self,
        params: ReverseGeocodingParams,
    ) -> ReqwestResult<GeocodingResponse> {
        let mut url = self
            .base_url
            .join("reverse")
            .expect("Failed to join URL path");

        let query = serde_urlencoded::to_string(&params).expect("Failed to serialize parameters");
        url.set_query(Some(&query));

        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<GeocodingResponse>()
            .await
    }

    /// Batch geocoding
    pub async fn batch_geocode(
        &self,
        queries: Vec<ForwardGeocodingParams>,
    ) -> ReqwestResult<GeocodingResponse> {
        let url = self
            .base_url
            .join("batch")
            .expect("Failed to join URL path");

        self.client
            .post(url)
            .json(&queries)
            .send()
            .await?
            .error_for_status()?
            .json::<GeocodingResponse>()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_forward_geocode() {
        let client = MapboxGeocodingClient::new("test_token".to_string());
        let params = ForwardGeocodingParams {
            q: "2 Lincoln Memorial Circle SW".to_string(),
            access_token: "test_token".to_string(),
            permanent: None,
            autocomplete: None,
            bbox: None,
            country: None,
            format: None,
            language: None,
            limit: None,
            proximity: None,
            types: None,
            worldview: None,
        };

        // This would need to be mocked for proper testing
        let _response = client.forward_geocode(params).await;
    }

    // Additional tests would be implemented here
}
