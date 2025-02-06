use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RwgpsWebhookCollection {
    id: i64,
    #[serde(rename = "type")]
    collection_type: String,
    url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RwgpsWebhookNotification {
    user_id: i64,
    item_type: String,
    item_id: i64,
    item_user_id: i64,
    item_url: String,
    action: String,
    collection: Option<RwgpsWebhookCollection>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RwgpsWebhookPayload {
    notifications: Vec<RwgpsWebhookNotification>,
}
