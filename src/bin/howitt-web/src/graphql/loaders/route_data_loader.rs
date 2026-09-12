use async_graphql::dataloader::Loader;
use futures::future::join_all;
use howitt::{
    models::{
        point::progress::{DistanceElevationProgress, DistanceProgress, Progress},
        route::{RouteId, RoutePoints, RoutePointsFilter},
    },
    repos::RoutePointsRepo,
    services::fetchers::CacheFetcher,
};
use howitt_client_types::CacheStore;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

/// Full-resolution points plus reusable calculations. Do not simplify here:
/// point counts, profile alignment and elevation totals must remain unchanged.
#[derive(Debug, Serialize, Deserialize)]
pub struct RouteData {
    pub route_points: RoutePoints,
    pub distance_points: Vec<f64>,
    pub elevation_ascent_m: f64,
    pub elevation_descent_m: f64,
}

impl From<RoutePoints> for RouteData {
    fn from(route_points: RoutePoints) -> Self {
        let totals = DistanceElevationProgress::last_from_points(route_points.points.clone());
        let distance_points = DistanceProgress::from_points(route_points.points.clone())
            .map(|progress| progress.distance_m)
            .collect();
        Self {
            route_points,
            distance_points,
            elevation_ascent_m: totals.as_ref().map(|p| p.elevation_gain_m).unwrap_or(0.0),
            elevation_descent_m: totals.as_ref().map(|p| p.elevation_loss_m).unwrap_or(0.0),
        }
    }
}

pub struct RouteDataLoader<Cache: CacheStore> {
    route_points_repo: RoutePointsRepo,
    cache: CacheFetcher<Cache>,
}

impl<Cache: CacheStore> RouteDataLoader<Cache> {
    pub fn new(route_points_repo: RoutePointsRepo, cache: Cache) -> Self {
        Self {
            route_points_repo,
            cache: CacheFetcher::new(cache),
        }
    }

    fn key(id: RouteId) -> String {
        format!("{id}#ROUTE_DATA")
    }
}

impl<Cache: CacheStore + Send + Sync + 'static> Loader<RouteId> for RouteDataLoader<Cache> {
    type Value = Arc<RouteData>;
    type Error = Arc<anyhow::Error>;

    async fn load(&self, keys: &[RouteId]) -> Result<HashMap<RouteId, Self::Value>, Self::Error> {
        let cached: HashMap<_, _> = join_all(keys.iter().map(|id| async {
            self.cache
                .get::<RouteData>(&Self::key(*id))
                .await
                .map(|data| (*id, Arc::new(data)))
        }))
        .await
        .into_iter()
        .flatten()
        .collect();
        let missing: Vec<_> = keys
            .iter()
            .copied()
            .filter(|id| !cached.contains_key(id))
            .collect();
        if missing.is_empty() {
            return Ok(cached);
        }

        // Wait for all cache lookups before one batched database read. Never
        // turn a cold route list into one PostgreSQL round-trip per cache miss.
        let fresh: Vec<_> = self
            .route_points_repo
            .filter_models(RoutePointsFilter::Ids(missing))
            .await
            .map_err(|error| Arc::new(error.into()))?
            .into_iter()
            .map(RouteData::from)
            .collect();
        join_all(fresh.iter().map(|data| async {
            self.cache.put(&Self::key(data.route_points.id), data).await;
        }))
        .await;

        Ok(cached
            .into_iter()
            .chain(
                fresh
                    .into_iter()
                    .map(|data| (data.route_points.id, Arc::new(data))),
            )
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Bytes;
    use howitt::{models::point::ElevationPoint, repos::Repo};
    use std::{sync::Mutex, time::Duration};

    #[derive(Clone, Default)]
    struct MemoryCache(Arc<Mutex<HashMap<String, Bytes>>>);

    #[async_trait::async_trait]
    impl CacheStore for MemoryCache {
        type Error = std::convert::Infallible;
        async fn get_bytes(&self, key: &str) -> Result<Option<Bytes>, Self::Error> {
            Ok(self.0.lock().unwrap().get(key).cloned())
        }
        async fn set_bytes(
            &self,
            key: &str,
            bytes: Bytes,
            ttl: Duration,
        ) -> Result<(), Self::Error> {
            assert_eq!(ttl, Duration::from_secs(3600));
            self.0.lock().unwrap().insert(key.into(), bytes);
            Ok(())
        }
    }

    #[derive(Debug)]
    struct BatchedRepo {
        rows: Vec<RoutePoints>,
        batches: Mutex<Vec<Vec<RouteId>>>,
    }

    #[async_trait::async_trait]
    impl Repo for BatchedRepo {
        type Model = RoutePoints;
        type Error = std::io::Error;
        async fn all(&self) -> Result<Vec<RoutePoints>, Self::Error> {
            panic!("must batch")
        }
        async fn get(&self, _: RouteId) -> Result<RoutePoints, Self::Error> {
            panic!("must batch")
        }
        async fn put(&self, _: RoutePoints) -> Result<(), Self::Error> {
            panic!("read only")
        }
        async fn filter_models(
            &self,
            filter: RoutePointsFilter,
        ) -> Result<Vec<RoutePoints>, Self::Error> {
            let RoutePointsFilter::Ids(ids) = filter;
            self.batches.lock().unwrap().push(ids.clone());
            Ok(self
                .rows
                .iter()
                .filter(|row| ids.contains(&row.id))
                .cloned()
                .collect())
        }
    }

    fn points(id: u128, elevations: &[f64]) -> RoutePoints {
        RoutePoints {
            id: uuid::Uuid::from_u128(id).into(),
            points: elevations
                .iter()
                .enumerate()
                .map(|(index, elevation)| ElevationPoint {
                    point: geo::Point::new(144.0 + index as f64 * 0.001, -37.0),
                    elevation: *elevation,
                })
                .collect(),
        }
    }

    #[test_case::test_case(vec![], 0.0, 0.0; "empty")]
    #[test_case::test_case(vec![100.0], 0.0, 0.0; "single point")]
    #[test_case::test_case(vec![100.0, 120.0, 110.0], 20.0, 10.0; "ascent and descent")]
    fn preserves_full_resolution(elevations: Vec<f64>, ascent: f64, descent: f64) {
        let source = points(1, &elevations);
        let data = RouteData::from(source.clone());
        assert_eq!(data.route_points, source);
        assert_eq!(data.elevation_ascent_m, ascent);
        assert_eq!(data.elevation_descent_m, descent);
        assert_eq!(data.distance_points.len(), elevations.len());
        assert!(data
            .distance_points
            .windows(2)
            .all(|pair| pair[1] >= pair[0]));
    }

    #[test]
    fn batches_only_misses_and_reuses_serialized_data_across_loaders() {
        futures::executor::block_on(async {
            let rows = vec![
                points(1, &[100.0, 120.0, 110.0]),
                points(2, &[]),
                points(3, &[0.0]),
            ];
            let ids: Vec<_> = rows.iter().map(|row| row.id).collect();
            let repo = Arc::new(BatchedRepo {
                rows,
                batches: Mutex::new(vec![]),
            });
            let cache = MemoryCache::default();
            let first = RouteDataLoader::new(repo.clone(), cache.clone())
                .load(&ids[..2])
                .await
                .unwrap();
            assert_eq!(
                repo.batches.lock().unwrap().as_slice(),
                &[ids[..2].to_vec()]
            );
            assert_eq!(first[&ids[0]].elevation_ascent_m, 20.0);
            // The next loader has no request-local state: two hits, one miss.
            let second = RouteDataLoader::new(repo.clone(), cache.clone())
                .load(&ids)
                .await
                .unwrap();
            assert_eq!(
                repo.batches.lock().unwrap().as_slice(),
                &[ids[..2].to_vec(), vec![ids[2]]]
            );
            assert_eq!(second[&ids[0]].route_points, first[&ids[0]].route_points);
            assert_eq!(
                second[&ids[0]].distance_points,
                first[&ids[0]].distance_points
            );
            let warm = RouteDataLoader::new(repo.clone(), cache.clone())
                .load(&ids)
                .await
                .unwrap();
            assert_eq!(warm.len(), 3);
            assert_eq!(repo.batches.lock().unwrap().len(), 2);
            assert!(cache
                .0
                .lock()
                .unwrap()
                .keys()
                .all(|key| key.starts_with("derived-v1:ROUTE#") && key.ends_with("#ROUTE_DATA")));
        });
    }
}
