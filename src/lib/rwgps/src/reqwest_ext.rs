use std::fmt::Write;

use serde::de::DeserializeOwned;
use serde_path_to_error::Segment;

use crate::RwgpsError;

#[derive(thiserror::Error, Debug)]
#[error(
    "RWGPS response decode failed: endpoint={endpoint} serde_path={path} serde_category={category}"
)]
pub struct SerdeDebugError {
    endpoint: &'static str,
    path: String,
    category: &'static str,
    #[source]
    error: serde_path_to_error::Error<serde_json::Error>,
}

impl SerdeDebugError {
    pub fn safe_diagnostic(&self) -> String {
        format!(
            "endpoint={} serde_path={} serde_category={}",
            self.endpoint, self.path, self.category
        )
    }
}

pub trait ResponseExt {
    async fn json_debug<T: DeserializeOwned>(self) -> Result<T, RwgpsError>;
}

impl ResponseExt for reqwest::Response {
    async fn json_debug<T: DeserializeOwned>(self) -> Result<T, RwgpsError> {
        let endpoint = endpoint_category(self.url());

        let bytes = self.bytes().await?;

        deserialize_debug(endpoint, &bytes)
    }
}

fn deserialize_debug<T: DeserializeOwned>(
    endpoint: &'static str,
    bytes: &[u8],
) -> Result<T, RwgpsError> {
    let jd = &mut serde_json::Deserializer::from_slice(bytes);

    match serde_path_to_error::deserialize(jd) {
        Ok(val) => Ok(val),
        Err(error) => {
            let path = safe_serde_path(error.path());
            let category = match error.inner().classify() {
                serde_json::error::Category::Io => "io",
                serde_json::error::Category::Syntax => "syntax",
                serde_json::error::Category::Data => "data",
                serde_json::error::Category::Eof => "eof",
            };
            Err(RwgpsError::from(SerdeDebugError {
                endpoint,
                path,
                category,
                error,
            }))
        }
    }
}

fn endpoint_category(url: &reqwest::Url) -> &'static str {
    let segments = url
        .path_segments()
        .map(|segments| segments.collect::<Vec<_>>())
        .unwrap_or_default();

    match segments.as_slice() {
        ["users", "current.json"] => "user",
        ["users", _, "routes.json"] => "user_routes",
        ["users", _, "trips.json"] => "user_trips",
        ["routes", _] => "route",
        ["trips", _] => "trip",
        _ => "unknown",
    }
}

fn safe_serde_path(path: &serde_path_to_error::Path) -> String {
    let mut output = String::new();

    for segment in path {
        match segment {
            Segment::Seq { index } => write!(output, "[{index}]").unwrap(),
            Segment::Map { key } if is_safe_field_name(key) => {
                if !output.is_empty() {
                    output.push('.');
                }
                output.push_str(key);
            }
            Segment::Map { .. } | Segment::Enum { .. } | Segment::Unknown => {
                if !output.is_empty() {
                    output.push('.');
                }
                output.push('?');
            }
        }
    }

    if output.is_empty() {
        output.push_str("root");
    }
    output
}

fn is_safe_field_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::{deserialize_debug, endpoint_category};
    use crate::RwgpsError;

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct Response {
        results: Vec<Record>,
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct Record {
        group_membership_id: usize,
    }

    #[test]
    fn safe_diagnostic_reports_endpoint_path_and_category_without_values_or_url() {
        let url = reqwest::Url::parse(
            "https://ridewithgps.com/users/123/routes.json?auth_token=secret-token&user=private",
        )
        .unwrap();
        let endpoint = endpoint_category(&url);
        let body = br#"{"results":[{"group_membership_id":"sensitive-response-value"}]}"#;

        let error = deserialize_debug::<Response>(endpoint, body).unwrap_err();
        let RwgpsError::SerdeDebug(error) = error else {
            panic!("expected response decode error");
        };
        let diagnostic = error.safe_diagnostic();

        assert_eq!(
            diagnostic,
            "endpoint=user_routes serde_path=results[0].group_membership_id serde_category=data"
        );
        assert!(!diagnostic.contains("sensitive-response-value"));
        assert!(!diagnostic.contains("secret-token"));
        assert!(!diagnostic.contains("private"));
        assert!(!diagnostic.contains("123"));
        assert!(!diagnostic.contains("https://"));
    }
}
