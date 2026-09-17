use std::sync::Arc;

use crate::models::{
    point::{ElevationPoint, TemporalElevationPoint},
    ride::Ride,
    route::Route,
};

pub type RwgpsSyncResult = Result<(), RwgpsSyncPersistenceError>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RwgpsSyncPersistenceErrorKind {
    Database,
    SerializationConversion,
    OwnershipConflict,
}

#[derive(Debug, thiserror::Error)]
#[error("RWGPS persistence failed")]
pub struct RwgpsSyncPersistenceError {
    kind: RwgpsSyncPersistenceErrorKind,
    #[source]
    source: Box<dyn std::error::Error + Send + Sync>,
}

impl RwgpsSyncPersistenceError {
    pub fn database(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::new(RwgpsSyncPersistenceErrorKind::Database, source)
    }

    pub fn serialization_conversion(
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::new(
            RwgpsSyncPersistenceErrorKind::SerializationConversion,
            source,
        )
    }

    pub fn ownership_conflict(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::new(RwgpsSyncPersistenceErrorKind::OwnershipConflict, source)
    }

    pub fn kind(&self) -> RwgpsSyncPersistenceErrorKind {
        self.kind
    }

    fn new(
        kind: RwgpsSyncPersistenceErrorKind,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind,
            source: Box::new(source),
        }
    }
}

#[async_trait::async_trait]
pub trait RwgpsSyncStore: Send + Sync {
    async fn save_route(&self, route: Route, points: Vec<ElevationPoint>) -> RwgpsSyncResult;

    async fn save_trip(&self, ride: Ride, points: Vec<TemporalElevationPoint>) -> RwgpsSyncResult;
}

pub type DynRwgpsSyncStore = Arc<dyn RwgpsSyncStore>;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::{RwgpsSyncPersistenceError, RwgpsSyncPersistenceErrorKind};

    #[test]
    fn persistence_error_preserves_its_typed_source() {
        let error = RwgpsSyncPersistenceError::serialization_conversion(std::io::Error::other(
            "private payload value",
        ));

        assert_eq!(
            error.kind(),
            RwgpsSyncPersistenceErrorKind::SerializationConversion
        );
        let source = error.source().expect("source is retained");
        assert!(source.downcast_ref::<std::io::Error>().is_some());
        assert_eq!(source.to_string(), "private payload value");
        assert_eq!(error.to_string(), "RWGPS persistence failed");
    }
}
