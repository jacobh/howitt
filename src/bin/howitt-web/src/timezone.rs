//! The same tzf-rs dataset and lookup order as DefaultFinder, loaded lazily from
//! static assets on Workers instead of embedding 13 MB into the Wasm module.
use tzf_rs::{Finder, FuzzyFinder};

pub struct TimezoneLookup {
    #[cfg(target_arch = "wasm32")]
    assets: worker::send::SendWrapper<worker::Fetcher>,
    #[cfg(target_arch = "wasm32")]
    initialization: futures::lock::Mutex<()>,
}

/// Only the lock is request-scoped; the cache contains completed, immutable data.
/// Recheck after locking because another resolver may have populated it while
/// this resolver was waiting. Failed/cancelled initialization remains retryable.
#[cfg(any(target_arch = "wasm32", test))]
async fn load_cached<T: 'static>(
    initialization: &futures::lock::Mutex<()>,
    cache: &'static std::thread::LocalKey<std::cell::RefCell<Option<std::rc::Rc<T>>>>,
    initialize: impl std::future::Future<Output = anyhow::Result<std::rc::Rc<T>>>,
) -> anyhow::Result<std::rc::Rc<T>> {
    if let Some(value) = cache.with(|cache| cache.borrow().clone()) {
        return Ok(value);
    }
    let _guard = initialization.lock().await;
    if let Some(value) = cache.with(|cache| cache.borrow().clone()) {
        return Ok(value);
    }
    let value = initialize.await?;
    cache.with(|cache| *cache.borrow_mut() = Some(value.clone()));
    Ok(value)
}

// Lookup order follows tzf-rs 0.4.11 DefaultFinder::get_tz_name (MIT,
// copyright 2022 ringsaturn; see docs/licenses/tzf-rs-MIT.txt).
fn lookup<'a>(finder: &'a Finder, fuzzy: &'a FuzzyFinder, lng: f64, lat: f64) -> &'a str {
    for dx in [0.0, -0.01, 0.01, -0.02, 0.02] {
        for dy in [0.0, -0.01, 0.01, -0.02, 0.02] {
            let fuzzy_name = fuzzy.get_tz_name(lng + dx, lat + dy);
            if !fuzzy_name.is_empty() {
                return fuzzy_name;
            }
            let name = finder.get_tz_name(lng + dx, lat + dy);
            if !name.is_empty() {
                return name;
            }
        }
    }
    ""
}

impl TimezoneLookup {
    #[cfg(target_arch = "wasm32")]
    pub fn new(assets: worker::Fetcher) -> Self {
        Self {
            assets: worker::send::SendWrapper::new(assets),
            initialization: futures::lock::Mutex::new(()),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_tz_name(&self, lng: f64, lat: f64) -> anyhow::Result<String> {
        #[cfg(target_arch = "wasm32")]
        {
            worker::send::SendFuture::new(self.lookup_assets(lng, lat)).await
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            static FINDER: std::sync::LazyLock<tzf_rs::DefaultFinder> =
                std::sync::LazyLock::new(tzf_rs::DefaultFinder::new);
            Ok(FINDER.get_tz_name(lng, lat).to_string())
        }
    }

    #[cfg(target_arch = "wasm32")]
    async fn asset(&self, name: &str) -> anyhow::Result<Vec<u8>> {
        let response = self
            .assets
            .fetch(format!("https://assets.invalid/timezones/{name}"), None)
            .await?;
        anyhow::ensure!(
            response.status().is_success(),
            "Timezone dataset asset is unavailable"
        );
        let bytes = axum::body::to_bytes(
            axum::body::Body::new(response.into_body()),
            16 * 1024 * 1024,
        )
        .await?;
        Ok(bytes.to_vec())
    }

    #[cfg(target_arch = "wasm32")]
    async fn lookup_assets(&self, lng: f64, lat: f64) -> anyhow::Result<String> {
        use std::{cell::RefCell, rc::Rc};
        // Cache only immutable parsed data. Never share in-flight I/O or request
        // bindings across requests: Workers disallows cross-request I/O reuse.
        thread_local! {
            static FUZZY: RefCell<Option<Rc<FuzzyFinder>>> = const { RefCell::new(None) };
            static POLYGONS: RefCell<Option<Rc<Finder>>> = const { RefCell::new(None) };
        }
        let fuzzy = load_cached(&self.initialization, &FUZZY, async {
            let bytes = self
                .asset("combined-with-oceans.reduce.preindex.pb")
                .await?;
            Ok(Rc::new(FuzzyFinder::from_pb(
                tzf_rs::r#gen::PreindexTimezones::try_from(bytes)?,
            )))
        })
        .await?;
        let direct = fuzzy.get_tz_name(lng, lat);
        if !direct.is_empty() {
            return Ok(direct.to_string());
        }
        let polygons = load_cached(&self.initialization, &POLYGONS, async {
            let bytes = self.asset("combined-with-oceans.reduce.pb").await?;
            Ok(Rc::new(Finder::from_pb(
                tzf_rs::r#gen::Timezones::try_from(bytes)?,
            )))
        })
        .await?;
        Ok(lookup(&polygons, &fuzzy, lng, lat).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{
        FutureExt,
        future::{join_all, poll_fn},
        lock::Mutex,
    };
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
        task::Poll,
    };

    async fn yield_once() {
        let mut yielded = false;
        poll_fn(|cx| {
            if yielded {
                return Poll::Ready(());
            }
            yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        })
        .await;
    }

    #[test]
    fn concurrent_lookups_initialize_each_dataset_once_and_reuse_completed_data() {
        thread_local! {
            static INDEX: RefCell<Option<Rc<u32>>> = const { RefCell::new(None) };
            static POLYGONS: RefCell<Option<Rc<u32>>> = const { RefCell::new(None) };
        }
        futures::executor::block_on(async {
            let gate = Mutex::new(());
            let index_loads = Cell::new(0);
            let polygon_loads = Cell::new(0);
            let results = join_all((0..64).map(|_| async {
                let index = load_cached(&gate, &INDEX, async {
                    index_loads.set(index_loads.get() + 1);
                    yield_once().await;
                    Ok(Rc::new(1))
                })
                .await
                .unwrap();
                let polygons = load_cached(&gate, &POLYGONS, async {
                    polygon_loads.set(polygon_loads.get() + 1);
                    yield_once().await;
                    Ok(Rc::new(2))
                })
                .await
                .unwrap();
                (index, polygons)
            }))
            .await;
            assert_eq!(index_loads.get(), 1);
            assert_eq!(polygon_loads.get(), 1);
            assert!(results.iter().all(|(index, polygons)| {
                Rc::ptr_eq(index, &results[0].0) && Rc::ptr_eq(polygons, &results[0].1)
            }));
            let next_request = Mutex::new(());
            let reused = load_cached(&next_request, &INDEX, async {
                panic!("completed dataset must be reused across requests")
            })
            .await
            .unwrap();
            assert!(Rc::ptr_eq(&reused, &results[0].0));
        });
    }

    #[test]
    fn failed_and_cancelled_initialization_can_retry() {
        thread_local! {
            static DATA: RefCell<Option<Rc<u32>>> = const { RefCell::new(None) };
        }
        futures::executor::block_on(async {
            let gate = Mutex::new(());
            let failed =
                load_cached(&gate, &DATA, async { anyhow::bail!("asset unavailable") }).await;
            assert!(failed.is_err());
            assert!(DATA.with(|cache| cache.borrow().is_none()));

            let mut cancelled = load_cached(&gate, &DATA, futures::future::pending()).boxed_local();
            assert!(futures::poll!(&mut cancelled).is_pending());
            drop(cancelled);
            assert!(
                gate.try_lock().is_some(),
                "cancellation must release the request lock"
            );
            assert!(DATA.with(|cache| cache.borrow().is_none()));
            let retried = load_cached(&gate, &DATA, async { Ok(Rc::new(42)) })
                .await
                .unwrap();
            assert_eq!(*retried, 42);
        });
    }

    #[test]
    fn asset_lookup_preserves_default_finder_results() {
        let finder = Finder::new();
        let fuzzy = FuzzyFinder::new();
        let original = tzf_rs::DefaultFinder::new();
        // Capitals, oceans, date line, state borders, and a coarse world grid.
        let points = [
            (144.96, -37.81),
            (151.21, -33.87),
            (153.02, -27.47),
            (159.08, -31.55),
            (141.0, -33.0),
            (129.0, -25.0),
            (180.0, 0.0),
            (-180.0, 0.0),
        ]
        .into_iter()
        .chain((-170..180).step_by(10).flat_map(|lng| {
            (-80..90)
                .step_by(10)
                .map(move |lat| (f64::from(lng), f64::from(lat)))
        }));
        for (lng, lat) in points {
            assert_eq!(
                lookup(&finder, &fuzzy, lng, lat),
                original.get_tz_name(lng, lat),
                "{lng},{lat}"
            );
        }
    }
}
