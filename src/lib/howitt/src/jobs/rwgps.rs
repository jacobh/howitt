use rwgps_types::webhook::RwgpsWebhookNotification;
use serde::{Deserialize, Serialize};

use crate::models::user::UserRwgpsConnection;

#[derive(Debug, Deserialize, Serialize)]
pub enum RwgpsJob {
    Webhook(RwgpsWebhookNotification),
    SyncTrip {
        rwgps_trip_id: usize,
        connection: UserRwgpsConnection,
    },
    SyncRoute {
        rwgps_route_id: usize,
        connection: UserRwgpsConnection,
    },
}
