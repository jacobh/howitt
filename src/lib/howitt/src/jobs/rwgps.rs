use derive_more::derive::From;
use rwgps_types::webhook::RwgpsWebhookNotification;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, From)]
pub enum RwgpsJob {
    Webhook(RwgpsWebhookNotification),
}
