use std::{
    ops::{Deref, DerefMut},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use async_trait::async_trait;
use futures_util::future::{select, Either};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tokio_postgres::Client;

use crate::PostgresRepoError;

const MAX_CONNECTIONS: usize = 5;
const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(10);

#[async_trait]
pub trait ConnectionFactory: Send + Sync {
    async fn connect(&self) -> Result<Client, PostgresRepoError>;
}

struct PoolState {
    idle: Mutex<Vec<Client>>,
    capacity: Arc<Semaphore>,
    factory: Arc<dyn ConnectionFactory>,
}

/// Bounded, lazy pool. Scope this to a request on Workers and a process natively.
#[derive(Clone)]
pub struct PostgresPool(Arc<PoolState>);

impl std::fmt::Debug for PostgresPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PostgresPool").finish_non_exhaustive()
    }
}

impl PostgresPool {
    pub fn new(factory: impl ConnectionFactory + 'static) -> Self {
        Self(Arc::new(PoolState {
            idle: Mutex::new(Vec::new()),
            capacity: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
            factory: Arc::new(factory),
        }))
    }

    pub(crate) async fn lease(&self) -> Result<ConnectionLease, PostgresRepoError> {
        let acquire = Box::pin(async {
            let permit = self
                .0
                .capacity
                .clone()
                .acquire_owned()
                .await
                .map_err(|_| PostgresRepoError::PoolClosed)?;
            let idle = self.0.idle.lock().expect("pool idle lock poisoned").pop();
            let client = match idle {
                Some(client) if !client.is_closed() => client,
                _ => self.0.factory.connect().await?,
            };
            Ok(ConnectionLease {
                client: Some(client),
                pool: self.0.clone(),
                _permit: permit,
                reusable: Arc::new(AtomicBool::new(true)),
            })
        });
        match select(
            acquire,
            Box::pin(futures_timer::Delay::new(ACQUIRE_TIMEOUT)),
        )
        .await
        {
            Either::Left((result, _)) => result,
            Either::Right(_) => Err(PostgresRepoError::AcquireTimeout),
        }
    }
}

/// Owns a connection until all statements (including a transaction) finish.
pub struct ConnectionLease {
    client: Option<Client>,
    pool: Arc<PoolState>,
    _permit: OwnedSemaphorePermit,
    pub(crate) reusable: Arc<AtomicBool>,
}

impl Deref for ConnectionLease {
    type Target = Client;
    fn deref(&self) -> &Client {
        self.client.as_ref().expect("leased client exists")
    }
}
impl DerefMut for ConnectionLease {
    fn deref_mut(&mut self) -> &mut Client {
        self.client.as_mut().expect("leased client exists")
    }
}
impl Drop for ConnectionLease {
    fn drop(&mut self) {
        if let Some(client) = self.client.take() {
            if self.reusable.load(Ordering::Relaxed) && !client.is_closed() {
                self.pool
                    .idle
                    .lock()
                    .expect("pool idle lock poisoned")
                    .push(client);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct FailingFactory(Arc<AtomicUsize>);
    #[async_trait]
    impl ConnectionFactory for FailingFactory {
        async fn connect(&self) -> Result<Client, PostgresRepoError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Err(PostgresRepoError::Connection("test failure".into()))
        }
    }

    #[tokio::test]
    async fn failed_connections_release_capacity() {
        let calls = Arc::new(AtomicUsize::new(0));
        let pool = PostgresPool::new(FailingFactory(calls.clone()));
        assert_eq!(calls.load(Ordering::SeqCst), 0, "connections must be lazy");
        for _ in 0..MAX_CONNECTIONS * 2 {
            assert!(matches!(
                pool.lease().await,
                Err(PostgresRepoError::Connection(_))
            ));
            assert_eq!(pool.0.capacity.available_permits(), MAX_CONNECTIONS);
        }
    }

    struct PendingFactory;
    #[async_trait]
    impl ConnectionFactory for PendingFactory {
        async fn connect(&self) -> Result<Client, PostgresRepoError> {
            std::future::pending().await
        }
    }

    #[tokio::test]
    async fn cancelled_connections_release_capacity() {
        let pool = PostgresPool::new(PendingFactory);
        {
            let acquire = pool.lease();
            tokio::pin!(acquire);
            assert!(futures_util::poll!(&mut acquire).is_pending());
            assert_eq!(pool.0.capacity.available_permits(), MAX_CONNECTIONS - 1);
        }
        assert_eq!(pool.0.capacity.available_permits(), MAX_CONNECTIONS);
    }

    #[tokio::test]
    async fn cancelled_waiters_do_not_leak_capacity() {
        let pool = PostgresPool::new(PendingFactory);
        let permits = pool
            .0
            .capacity
            .clone()
            .acquire_many_owned(MAX_CONNECTIONS as u32)
            .await
            .unwrap();
        {
            let acquire = pool.lease();
            tokio::pin!(acquire);
            assert!(futures_util::poll!(&mut acquire).is_pending());
        }
        drop(permits);
        assert_eq!(pool.0.capacity.available_permits(), MAX_CONNECTIONS);
        {
            let acquire = pool.lease();
            tokio::pin!(acquire);
            assert!(futures_util::poll!(&mut acquire).is_pending());
            assert_eq!(pool.0.capacity.available_permits(), MAX_CONNECTIONS - 1);
        }
        assert_eq!(pool.0.capacity.available_permits(), MAX_CONNECTIONS);
    }
}
