//! The same tzf-rs dataset and lookup behavior as DefaultFinder, loaded from a
//! static asset on Workers instead of embedding it into the Wasm module.
#[cfg(not(target_arch = "wasm32"))]
use tzf_rs::DefaultFinder;
#[cfg(target_arch = "wasm32")]
use tzf_rs::EmbeddedFinder;

pub struct TimezoneLookup {
    #[cfg(target_arch = "wasm32")]
    assets: worker::send::SendWrapper<worker::Fetcher>,
    #[cfg(target_arch = "wasm32")]
    finder: async_once_cell::OnceCell<worker::send::SendWrapper<std::rc::Rc<EmbeddedFinder>>>,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static FINDER: std::cell::RefCell<Option<std::rc::Rc<EmbeddedFinder>>> = const {
        std::cell::RefCell::new(None)
    };
}

#[cfg(target_arch = "wasm32")]
async fn load(assets: &worker::Fetcher) -> anyhow::Result<std::rc::Rc<EmbeddedFinder>> {
    if let Some(finder) = FINDER.with(|finder| finder.borrow().clone()) {
        return Ok(finder);
    }
    let response = assets
        .fetch("https://assets.invalid/timezones/lite.tzb", None)
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
    let finder = std::rc::Rc::new(EmbeddedFinder::from_tzb(bytes.to_vec())?);
    FINDER.with(|cache| *cache.borrow_mut() = Some(finder.clone()));
    Ok(finder)
}

impl TimezoneLookup {
    #[cfg(target_arch = "wasm32")]
    pub fn new(assets: worker::Fetcher) -> Self {
        Self {
            assets: worker::send::SendWrapper::new(assets),
            finder: async_once_cell::OnceCell::new(),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_tz_name(&self, lng: f64, lat: f64) -> anyhow::Result<String> {
        #[cfg(target_arch = "wasm32")]
        {
            let finder = self
                .finder
                .get_or_try_init(worker::send::SendFuture::new(async {
                    Ok::<_, anyhow::Error>(worker::send::SendWrapper::new(
                        load(&self.assets).await?,
                    ))
                }))
                .await?;
            Ok(finder.get_tz_name(lng, lat).to_string())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            static FINDER: std::sync::LazyLock<DefaultFinder> =
                std::sync::LazyLock::new(DefaultFinder::new);
            Ok(FINDER.get_tz_name(lng, lat).to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use tzf_rs::DefaultFinder;

    #[test]
    fn default_finder_preserves_representative_lookup_results() {
        let finder = DefaultFinder::new();
        assert_eq!(finder.get_tz_name(144.96, -37.81), "Australia/Melbourne");
        assert_eq!(finder.get_tz_name(151.21, -33.87), "Australia/Sydney");
        assert_eq!(finder.get_tz_name(153.02, -27.47), "Australia/Brisbane");
        assert_eq!(finder.get_tz_name(159.08, -31.55), "Australia/Lord_Howe");
        assert_eq!(finder.get_tz_name(141.0, -33.0), "Australia/Adelaide");
    }
}
