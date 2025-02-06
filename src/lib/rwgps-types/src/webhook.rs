use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RwgpsWebhookCollection {
    pub id: i64,
    #[serde(rename = "type")]
    pub collection_type: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Route,
    Trip,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Created,
    Updated,
    Deleted,
    Added,
    Removed,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RwgpsWebhookNotification {
    pub user_id: i64,
    pub item_type: ItemType,
    pub item_id: i64,
    pub item_user_id: i64,
    pub item_url: String,
    pub action: Action,
    pub collection: Option<RwgpsWebhookCollection>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RwgpsWebhookPayload {
    pub notifications: Vec<RwgpsWebhookNotification>,
}
