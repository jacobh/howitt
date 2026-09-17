use howitt::{
    models::{
        external_ref::{ExternalId, RwgpsId},
        point::{ElevationPoint, TemporalElevationPoint},
        ride::Ride,
        route::Route,
    },
    services::sync::rwgps_v2::persistence::{
        RwgpsSyncPersistenceError, RwgpsSyncResult, RwgpsSyncStore,
    },
};
use tokio_postgres::types::Type;

use crate::PostgresPool;

#[derive(Debug, Clone)]
pub struct PostgresRwgpsSyncStore {
    pool: PostgresPool,
}

impl PostgresRwgpsSyncStore {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }
}

fn invalid_input(message: &'static str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, message)
}

fn route_external_id(route: &Route) -> Result<i32, RwgpsSyncPersistenceError> {
    match route.external_ref.as_ref().map(|reference| &reference.id) {
        Some(ExternalId::Rwgps(RwgpsId::Route(id))) => (*id)
            .try_into()
            .map_err(RwgpsSyncPersistenceError::serialization_conversion),
        _ => Err(RwgpsSyncPersistenceError::serialization_conversion(
            invalid_input("RWGPS route persistence requires a route external reference"),
        )),
    }
}

fn trip_external_id(ride: &Ride) -> Result<i32, RwgpsSyncPersistenceError> {
    match ride.external_ref.as_ref().map(|reference| &reference.id) {
        Some(ExternalId::Rwgps(RwgpsId::Trip(id))) => (*id)
            .try_into()
            .map_err(RwgpsSyncPersistenceError::serialization_conversion),
        _ => Err(RwgpsSyncPersistenceError::serialization_conversion(
            invalid_input("RWGPS trip persistence requires a trip external reference"),
        )),
    }
}

#[async_trait::async_trait]
impl RwgpsSyncStore for PostgresRwgpsSyncStore {
    async fn save_route(&self, route: Route, points: Vec<ElevationPoint>) -> RwgpsSyncResult {
        let external_id = route_external_id(&route)?;
        let external_ref = serde_json::to_value(&route.external_ref)
            .map_err(RwgpsSyncPersistenceError::serialization_conversion)?;
        let sample_points = route
            .sample_points
            .as_ref()
            .map(serde_json::to_value)
            .transpose()
            .map_err(RwgpsSyncPersistenceError::serialization_conversion)?;
        let points = serde_json::to_value(points)
            .map_err(RwgpsSyncPersistenceError::serialization_conversion)?;
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(RwgpsSyncPersistenceError::database)?;
        let tx = conn
            .transaction()
            .await
            .map_err(RwgpsSyncPersistenceError::database)?;
        tx.query_typed(
            "select pg_advisory_xact_lock($1, $2)",
            &[(&1_i32, Type::INT4), (&external_id, Type::INT4)],
        )
        .await
        .map_err(RwgpsSyncPersistenceError::database)?;
        let existing = tx
            .query_typed_opt(
                "select id, user_id, external_ref from routes where (external_ref->'id'->'Rwgps'->'Route')::int = $1",
                &[(&external_id, Type::INT4)],
            )
            .await
            .map_err(RwgpsSyncPersistenceError::database)?;
        let id = if let Some(row) = existing {
            let user_id: uuid::Uuid = row
                .try_get("user_id")
                .map_err(RwgpsSyncPersistenceError::database)?;
            if user_id != *route.user_id.as_uuid() {
                return Err(RwgpsSyncPersistenceError::ownership_conflict(
                    invalid_input("RWGPS route external reference belongs to another user"),
                ));
            }
            let stored: serde_json::Value = row
                .try_get("external_ref")
                .map_err(RwgpsSyncPersistenceError::database)?;
            let stored: howitt::models::external_ref::ExternalRef = serde_json::from_value(stored)
                .map_err(RwgpsSyncPersistenceError::serialization_conversion)?;
            let incoming = route.external_ref.as_ref().expect("validated above");
            // Equal timestamps may be a repair or a newer sync-code version.
            if stored.updated_at > incoming.updated_at {
                tx.commit()
                    .await
                    .map_err(RwgpsSyncPersistenceError::database)?;
                return Ok(());
            }
            let id: uuid::Uuid = row
                .try_get("id")
                .map_err(RwgpsSyncPersistenceError::database)?;
            tx.execute_typed(
                "update routes set external_ref=$2, sample_points=$3, distance_m=$4 where id=$1",
                &[
                    (&id, Type::UUID),
                    (&external_ref, Type::JSONB),
                    (&sample_points, Type::JSONB),
                    (&(route.distance as i32), Type::INT4),
                ],
            )
            .await
            .map_err(RwgpsSyncPersistenceError::database)?;
            id
        } else {
            let id = *route.id.as_uuid();
            tx.execute_typed(
                "insert into routes (id, created_at, name, slug, external_ref, sample_points, distance_m, user_id) values ($1, now(), $2, $3, $4, $5, $6, $7)",
                &[
                    (&id, Type::UUID), (&route.name, Type::TEXT), (&route.slug, Type::VARCHAR),
                    (&external_ref, Type::JSONB), (&sample_points, Type::JSONB),
                    (&(route.distance as i32), Type::INT4), (route.user_id.as_uuid(), Type::UUID),
                ],
            )
            .await
            .map_err(RwgpsSyncPersistenceError::database)?;
            id
        };
        tx.execute_typed(
            "insert into route_points (route_id, points) values ($1, $2) on conflict (route_id) do update set points=excluded.points",
            &[(&id, Type::UUID), (&points, Type::JSONB)],
        )
        .await
        .map_err(RwgpsSyncPersistenceError::database)?;
        tx.commit()
            .await
            .map_err(RwgpsSyncPersistenceError::database)?;
        Ok(())
    }

    async fn save_trip(&self, ride: Ride, points: Vec<TemporalElevationPoint>) -> RwgpsSyncResult {
        let external_id = trip_external_id(&ride)?;
        let external_ref = serde_json::to_value(&ride.external_ref)
            .map_err(RwgpsSyncPersistenceError::serialization_conversion)?;
        let points = serde_json::to_value(points)
            .map_err(RwgpsSyncPersistenceError::serialization_conversion)?;
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(RwgpsSyncPersistenceError::database)?;
        let tx = conn
            .transaction()
            .await
            .map_err(RwgpsSyncPersistenceError::database)?;
        tx.query_typed(
            "select pg_advisory_xact_lock($1, $2)",
            &[(&2_i32, Type::INT4), (&external_id, Type::INT4)],
        )
        .await
        .map_err(RwgpsSyncPersistenceError::database)?;
        let existing = tx.query_typed_opt(
            "select id, user_id, external_ref from rides where (external_ref->'id'->'Rwgps'->'Trip')::int = $1",
            &[(&external_id, Type::INT4)],
        )
        .await
        .map_err(RwgpsSyncPersistenceError::database)?;
        let id = if let Some(row) = existing {
            let user_id: uuid::Uuid = row
                .try_get("user_id")
                .map_err(RwgpsSyncPersistenceError::database)?;
            if user_id != *ride.user_id.as_uuid() {
                return Err(RwgpsSyncPersistenceError::ownership_conflict(
                    invalid_input("RWGPS trip external reference belongs to another user"),
                ));
            }
            let stored: serde_json::Value = row
                .try_get("external_ref")
                .map_err(RwgpsSyncPersistenceError::database)?;
            let stored: howitt::models::external_ref::ExternalRef = serde_json::from_value(stored)
                .map_err(RwgpsSyncPersistenceError::serialization_conversion)?;
            let incoming = ride.external_ref.as_ref().expect("validated above");
            if stored.updated_at > incoming.updated_at {
                tx.commit()
                    .await
                    .map_err(RwgpsSyncPersistenceError::database)?;
                return Ok(());
            }
            let id: uuid::Uuid = row
                .try_get("id")
                .map_err(RwgpsSyncPersistenceError::database)?;
            // Name and distance are intentionally user-owned after initial import.
            tx.execute_typed(
                "update rides set external_ref=$2, started_at=$3, finished_at=$4 where id=$1",
                &[
                    (&id, Type::UUID),
                    (&external_ref, Type::JSONB),
                    (&ride.started_at, Type::TIMESTAMPTZ),
                    (&ride.finished_at, Type::TIMESTAMPTZ),
                ],
            )
            .await
            .map_err(RwgpsSyncPersistenceError::database)?;
            id
        } else {
            let id = *ride.id.as_uuid();
            tx.execute_typed(
                "insert into rides (id, name, created_at, external_ref, distance_m, started_at, finished_at, user_id) values ($1, $2, now(), $3, $4, $5, $6, $7)",
                &[(&id, Type::UUID), (&ride.name, Type::VARCHAR), (&external_ref, Type::JSONB), (&(ride.distance as i32), Type::INT4), (&ride.started_at, Type::TIMESTAMPTZ), (&ride.finished_at, Type::TIMESTAMPTZ), (ride.user_id.as_uuid(), Type::UUID)],
            )
            .await
            .map_err(RwgpsSyncPersistenceError::database)?;
            id
        };
        tx.execute_typed(
            "insert into ride_points (ride_id, points) values ($1, $2) on conflict (ride_id) do update set points=excluded.points",
            &[(&id, Type::UUID), (&points, Type::JSONB)],
        )
        .await
        .map_err(RwgpsSyncPersistenceError::database)?;
        tx.commit()
            .await
            .map_err(RwgpsSyncPersistenceError::database)?;
        Ok(())
    }
}
