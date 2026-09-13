use rwgps_types::webhook::RwgpsWebhookNotification;
use serde::{Deserialize, Serialize};

use crate::models::user::UserId;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum RwgpsJob {
    Webhook(RwgpsWebhookNotification),
    SyncTrip {
        rwgps_trip_id: usize,
        user_id: UserId,
    },
    SyncRoute {
        rwgps_route_id: usize,
        user_id: UserId,
    },
    SyncHistory {
        user_id: UserId,
    },
}
