use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RwgpsWebhookCollection {
    pub id: i64,
    #[serde(rename = "type")]
    pub collection_type: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RwgpsWebhookNotification {
    pub user_id: i64,
    pub item_type: String,
    pub item_id: i64,
    pub item_user_id: i64,
    pub item_url: String,
    pub action: String,
    pub collection: Option<RwgpsWebhookCollection>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RwgpsWebhookPayload {
    pub notifications: Vec<RwgpsWebhookNotification>,
}
