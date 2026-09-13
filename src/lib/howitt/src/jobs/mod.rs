use derive_more::derive::From;
use serde::{Deserialize, Serialize};

pub mod media;
pub mod rwgps;

#[derive(Debug, Deserialize, Serialize, From, Clone)]
pub enum Job {
    Media(media::MediaJob),
    Rwgps(rwgps::RwgpsJob),
}

/// Version the wire contract independently of the consumer deployment.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "version", content = "job")]
pub enum QueueMessage {
    #[serde(rename = "1")]
    V1(Job),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_contract_contains_ids_and_rejects_unknown_versions() {
        let wire = serde_json::json!({
            "version": "1",
            "job": {"Rwgps": {"SyncRoute": {
                "rwgps_route_id": 42,
                "user_id": "USER#00000000-0000-0000-0000-000000000007"
            }}}
        });
        let message: QueueMessage = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(serde_json::to_value(message).unwrap(), wire);
        let mut future = wire;
        future["version"] = "2".into();
        assert!(serde_json::from_value::<QueueMessage>(future).is_err());
        assert!(serde_json::from_value::<QueueMessage>(serde_json::json!({
            "version": "1", "job": {"Media": {"Process": "MEDIA#00000000-0000-0000-0000-000000000007"}}
        })).is_err());
    }
}
