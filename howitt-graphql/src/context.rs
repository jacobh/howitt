use howitt::repos::{CheckpointRepo, ConfigRepo, RideModelRepo, RouteModelRepo};

use crate::credentials::Credentials;

pub struct SchemaData {
    pub config_repo: ConfigRepo,
    pub checkpoint_repo: CheckpointRepo,
    pub route_repo: RouteModelRepo,
    pub ride_repo: RideModelRepo,
}

pub struct RequestData {
    pub credentials: Option<Credentials>,
}
