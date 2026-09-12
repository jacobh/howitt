//! The same tzf-rs dataset and lookup order as DefaultFinder, loaded lazily from
//! static assets on Workers instead of embedding 13 MB into the Wasm module.
use tzf_rs::{Finder, FuzzyFinder};

pub struct TimezoneLookup {
    #[cfg(target_arch = "wasm32")]
    assets: worker::send::SendWrapper<worker::Fetcher>,
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
        let fuzzy = match FUZZY.with(|cache| cache.borrow().clone()) {
            Some(finder) => finder,
            None => {
                let bytes = self
                    .asset("combined-with-oceans.reduce.preindex.pb")
                    .await?;
                let finder = Rc::new(FuzzyFinder::from_pb(
                    tzf_rs::gen::PreindexTimezones::try_from(bytes)?,
                ));
                FUZZY.with(|cache| *cache.borrow_mut() = Some(finder.clone()));
                finder
            }
        };
        let direct = fuzzy.get_tz_name(lng, lat);
        if !direct.is_empty() {
            return Ok(direct.to_string());
        }
        let polygons = match POLYGONS.with(|cache| cache.borrow().clone()) {
            Some(finder) => finder,
            None => {
                let bytes = self.asset("combined-with-oceans.reduce.pb").await?;
                let finder = Rc::new(Finder::from_pb(tzf_rs::gen::Timezones::try_from(bytes)?));
                POLYGONS.with(|cache| *cache.borrow_mut() = Some(finder.clone()));
                finder
            }
        };
        Ok(lookup(&polygons, &fuzzy, lng, lat).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
