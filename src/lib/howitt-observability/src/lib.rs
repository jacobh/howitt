//! Request-owned spans with async-context restoration on every Rust future poll.
//! Callers must supply schema/static metadata only, never input values or errors.
use std::future::Future;

pub struct TraceSpan {
    outcome_key: &'static str,
    #[cfg(target_arch = "wasm32")]
    span: Option<worker::send::SendWrapper<worker::wasm_bindgen::JsValue>>,
    #[cfg(not(target_arch = "wasm32"))]
    span: Option<tracing::Span>,
}

impl TraceSpan {
    pub fn new(name: &str, outcome_key: &'static str) -> Self {
        #[cfg(target_arch = "wasm32")]
        let span = {
            let span = wasm::start(name);
            (!span.is_null()).then(|| worker::send::SendWrapper::new(span))
        };
        #[cfg(not(target_arch = "wasm32"))]
        let span = Some(tracing::info_span!(
            "howitt.operation",
            operation.name = name,
            graphql.parent_type = tracing::field::Empty,
            graphql.field.name = tracing::field::Empty,
            graphql.return_type = tracing::field::Empty,
            graphql.outcome = tracing::field::Empty,
            repo.model = tracing::field::Empty,
            repo.operation = tracing::field::Empty,
            repo.outcome = tracing::field::Empty,
            db.system.name = tracing::field::Empty,
            db.operation.name = tracing::field::Empty,
            db.outcome = tracing::field::Empty,
        ));
        let this = Self { span, outcome_key };
        this.attribute(outcome_key, "cancelled");
        this
    }

    pub fn attribute(&self, key: &'static str, value: &str) {
        if let Some(span) = &self.span {
            #[cfg(target_arch = "wasm32")]
            wasm::attribute(span, key, value);
            #[cfg(not(target_arch = "wasm32"))]
            span.record(key, value);
        }
    }

    pub async fn trace<T, E>(mut self, future: impl Future<Output = Result<T, E>>) -> Result<T, E> {
        let result = self.scope(future).await;
        self.finish(if result.is_ok() { "ok" } else { "error" });
        result
    }

    pub async fn scope<T>(&self, future: impl Future<Output = T>) -> T {
        let Some(span) = &self.span else {
            return future.await;
        };
        #[cfg(not(target_arch = "wasm32"))]
        {
            use tracing::Instrument;
            future.instrument(span.clone()).await
        }
        #[cfg(target_arch = "wasm32")]
        {
            let mut future = std::pin::pin!(future);
            std::future::poll_fn(|cx| {
                let mut result = std::task::Poll::Pending;
                wasm::poll(span, &mut || {
                    result = future.as_mut().poll(cx);
                });
                result
            })
            .await
        }
    }

    pub fn complete(mut self, outcome: &str) {
        self.finish(outcome);
    }

    fn finish(&mut self, outcome: &str) {
        self.attribute(self.outcome_key, outcome);
        if let Some(span) = self.span.take() {
            #[cfg(target_arch = "wasm32")]
            wasm::end(&span);
            #[cfg(not(target_arch = "wasm32"))]
            drop(span);
        }
    }
}

impl Drop for TraceSpan {
    fn drop(&mut self) {
        self.finish("cancelled");
    }
}

/// wasm-bindgen's shared task queue does not preserve each spawned task's JS
/// context. Capture it at spawn time and restore it on every poll (DataLoaders
/// otherwise inherit whichever unrelated task wakes the queue).
#[cfg(target_arch = "wasm32")]
pub fn spawn_local(future: impl Future<Output = ()> + 'static) {
    let context = wasm::capture();
    wasm_bindgen_futures::spawn_local(async move {
        let mut future = std::pin::pin!(future);
        std::future::poll_fn(|cx| {
            let mut result = std::task::Poll::Pending;
            wasm::poll(&context, &mut || {
                result = future.as_mut().poll(cx);
            });
            result
        })
        .await;
    });
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use worker::wasm_bindgen::{self, prelude::*};
    #[wasm_bindgen(module = "/src/span.js")]
    extern "C" {
        #[wasm_bindgen(js_name = captureContext)]
        pub fn capture() -> JsValue;
        #[wasm_bindgen(js_name = startSpan)]
        pub fn start(name: &str) -> JsValue;
        #[wasm_bindgen(js_name = setSpanAttribute)]
        pub fn attribute(span: &JsValue, key: &str, value: &str);
        #[wasm_bindgen(js_name = pollSpan)]
        pub fn poll(span: &JsValue, callback: &mut dyn FnMut());
        #[wasm_bindgen(js_name = endSpan)]
        pub fn end(span: &JsValue);
    }
}
