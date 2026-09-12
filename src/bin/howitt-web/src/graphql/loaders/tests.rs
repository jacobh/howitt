use super::{
    ride_loader::RideLoader, trip_content_loader::TripRidesLoader, user_loader::UserLoader,
};
use async_graphql::dataloader::{DataLoader, HashMapCache};
use howitt::models::{
    ride::{Ride, RideFilter, RideId},
    trip::TripId,
};
use howitt::{
    models::user::{User, UserFilter, UserId},
    repos::Repo,
};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Users {
    user: User,
    batches: Mutex<Vec<Vec<UserId>>>,
}

#[async_trait::async_trait]
impl Repo for Users {
    type Model = User;
    type Error = std::io::Error;
    async fn all(&self) -> Result<Vec<User>, Self::Error> {
        panic!("must batch")
    }
    async fn get(&self, _: UserId) -> Result<User, Self::Error> {
        panic!("must batch")
    }
    async fn put(&self, _: User) -> Result<(), Self::Error> {
        panic!("read only")
    }
    async fn filter_models(&self, filter: UserFilter) -> Result<Vec<User>, Self::Error> {
        let UserFilter::Ids(ids) = filter else {
            panic!("must batch IDs")
        };
        self.batches.lock().unwrap().push(ids.clone());
        Ok(if ids.contains(&self.user.id) {
            vec![self.user.clone()]
        } else {
            vec![]
        })
    }
}

#[derive(Debug)]
struct Rides {
    rows: Mutex<Vec<Ride>>,
    queries: Mutex<usize>,
}

#[async_trait::async_trait]
impl Repo for Rides {
    type Model = Ride;
    type Error = std::io::Error;
    async fn all(&self) -> Result<Vec<Ride>, Self::Error> {
        panic!("must filter")
    }
    async fn get(&self, _: RideId) -> Result<Ride, Self::Error> {
        panic!("must filter")
    }
    async fn put(&self, _: Ride) -> Result<(), Self::Error> {
        panic!("read only")
    }
    async fn filter_models(&self, filter: RideFilter) -> Result<Vec<Ride>, Self::Error> {
        assert!(matches!(
            filter,
            RideFilter::ForTrip(_) | RideFilter::Ids(_)
        ));
        *self.queries.lock().unwrap() += 1;
        Ok(self.rows.lock().unwrap().clone())
    }
}

#[tokio::test]
async fn trip_rides_cache_empty_results_prime_entities_and_refresh_after_changes() {
    let repo = Arc::new(Rides {
        rows: Mutex::new(vec![]),
        queries: Mutex::new(0),
    });
    let trips = DataLoader::with_cache(
        TripRidesLoader(repo.clone()),
        tokio::spawn,
        HashMapCache::default(),
    );
    let rides = DataLoader::with_cache(
        RideLoader::new(repo.clone()),
        tokio::spawn,
        HashMapCache::default(),
    );
    let trip = TripId::new();
    let (a, b) = tokio::join!(trips.load_one(trip), trips.load_one(trip));
    assert!(a.unwrap().unwrap().is_empty());
    assert!(b.unwrap().unwrap().is_empty());
    trips.load_one(trip).await.unwrap();
    assert_eq!(*repo.queries.lock().unwrap(), 1);

    let ride = Ride {
        id: RideId::new(),
        name: "test ride".into(),
        user_id: UserId::new(),
        distance: 100.0,
        started_at: chrono::Utc::now(),
        finished_at: chrono::Utc::now(),
        external_ref: None,
    };
    repo.rows.lock().unwrap().push(ride.clone());
    trips.clear::<TripId>();
    let rows = trips.load_one(trip).await.unwrap().unwrap();
    assert_eq!(rows, vec![ride.clone()]);
    rides
        .feed_many(rows.into_iter().map(|row| (row.id, row)))
        .await;
    assert_eq!(rides.load_one(ride.id).await.unwrap(), Some(ride));
    assert_eq!(*repo.queries.lock().unwrap(), 2);
}

#[tokio::test]
async fn users_batch_cache_prime_and_invalidate_within_one_request() {
    let user = User {
        id: UserId::new(),
        username: "test".into(),
        email: "test@example.invalid".into(),
        password: howitt::services::user::password::hash_password("test-password").unwrap(),
        created_at: chrono::Utc::now(),
        rwgps_connection: None,
    };
    let repo = Arc::new(Users {
        user: user.clone(),
        batches: Mutex::new(vec![]),
    });
    let loader = DataLoader::with_cache(
        UserLoader::new(repo.clone()),
        tokio::spawn,
        HashMapCache::default(),
    );
    let (a, b) = tokio::join!(loader.load_one(user.id), loader.load_one(user.id));
    assert_eq!(a.unwrap().unwrap().id, user.id);
    assert_eq!(b.unwrap().unwrap().id, user.id);
    assert_eq!(repo.batches.lock().unwrap().len(), 1);
    loader.load_one(user.id).await.unwrap();
    assert_eq!(repo.batches.lock().unwrap().len(), 1);

    loader.clear::<UserId>();
    loader.feed_one(user.id, user.clone()).await;
    loader.load_one(user.id).await.unwrap();
    assert_eq!(
        repo.batches.lock().unwrap().len(),
        1,
        "primed users need no query"
    );
    loader.clear::<UserId>();
    loader.load_one(user.id).await.unwrap();
    assert_eq!(repo.batches.lock().unwrap().len(), 2);

    let next_request = DataLoader::with_cache(
        UserLoader::new(repo.clone()),
        tokio::spawn,
        HashMapCache::default(),
    );
    next_request.load_one(user.id).await.unwrap();
    assert_eq!(
        repo.batches.lock().unwrap().len(),
        3,
        "cache must not cross requests"
    );
}
