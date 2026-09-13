#[cfg(test)]
mod tests {
    use howitt_web::ResolverTracing;
    use std::sync::Arc;
    use std::{collections::HashMap, sync::Mutex};
    use tracing::{
        Subscriber,
        field::{Field, Visit},
        span::{Attributes, Id, Record},
    };
    use tracing_subscriber::{
        Layer,
        layer::{Context, SubscriberExt},
        registry::LookupSpan,
    };

    #[derive(Debug, Default)]
    struct Values(HashMap<String, String>);
    impl Visit for Values {
        fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
            self.0.insert(field.name().into(), format!("{value:?}"));
        }
        fn record_str(&mut self, field: &Field, value: &str) {
            self.0.insert(field.name().into(), value.into());
        }
    }
    #[derive(Debug)]
    struct Captured {
        id: u64,
        parent: Option<u64>,
        values: Values,
        closed: bool,
    }
    #[derive(Clone, Default)]
    struct Capture(Arc<Mutex<Vec<Captured>>>);
    impl<S: Subscriber + for<'a> LookupSpan<'a>> Layer<S> for Capture {
        fn register_callsite(
            &self,
            _: &'static tracing::Metadata<'static>,
        ) -> tracing::subscriber::Interest {
            tracing::subscriber::Interest::sometimes()
        }
        fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
            let mut values = Values::default();
            attrs.record(&mut values);
            self.0.lock().unwrap().push(Captured {
                id: id.into_u64(),
                parent: ctx
                    .span(id)
                    .and_then(|span| span.parent())
                    .map(|span| span.id().into_u64()),
                values,
                closed: false,
            });
        }
        fn on_record(&self, id: &Id, values: &Record<'_>, _: Context<'_, S>) {
            let mut records = self.0.lock().unwrap();
            let record = records
                .iter_mut()
                .find(|record| record.id == id.into_u64())
                .unwrap();
            values.record(&mut record.values);
        }
        fn on_close(&self, id: Id, _: Context<'_, S>) {
            self.0
                .lock()
                .unwrap()
                .iter_mut()
                .find(|record| record.id == id.into_u64())
                .unwrap()
                .closed = true;
        }
    }

    struct Query;
    struct Item;
    #[async_graphql::Object]
    impl Item {
        async fn value(&self) -> i32 {
            let mut yielded = false;
            futures::future::poll_fn(|cx| {
                if yielded {
                    return std::task::Poll::Ready(7);
                }
                yielded = true;
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            })
            .await
        }
    }
    #[async_graphql::Object]
    impl Query {
        async fn items(&self) -> Vec<Item> {
            vec![Item, Item]
        }
        async fn points(&self) -> Vec<Vec<i32>> {
            vec![vec![1, 2], vec![3, 4]]
        }
        async fn failure(&self, secret: String) -> async_graphql::Result<Option<String>> {
            let _ = secret;
            Err(async_graphql::Error::new("sensitive error message"))
        }
    }

    #[test]
    fn traces_fields_and_errors_without_values_aliases_or_list_items() {
        let capture = Capture::default();
        let subscriber = tracing_subscriber::registry().with(capture.clone());
        let _guard = tracing::subscriber::set_default(subscriber);
        tracing::callsite::rebuild_interest_cache();
        let schema = async_graphql::Schema::build(
            Query,
            async_graphql::EmptyMutation,
            async_graphql::EmptySubscription,
        )
        .extension(ResolverTracing)
        .finish();
        let result = futures::executor::block_on(
            schema.execute("query PrivateOperation { secretAlias: items { value } points }"),
        );
        assert!(result.errors.is_empty());
        assert_eq!(
            result.data,
            async_graphql::value!({
                "secretAlias": [{"value": 7}, {"value": 7}], "points": [[1, 2], [3, 4]]
            })
        );
        let failure = futures::executor::block_on(
            schema.execute(r#"{ failure(secret: "sensitive-argument") }"#),
        );
        assert_eq!(failure.errors.len(), 1);
        {
            let captured = capture.0.lock().unwrap();
            assert!(captured.iter().all(|record| record.closed));
            let records: Vec<_> = captured
                .iter()
                .filter(|record| record.values.0.contains_key("graphql.field.name"))
                .collect();
            assert_eq!(records.len(), 5, "only field resolvers, not list elements");
            let items = records
                .iter()
                .find(|r| r.values.0["graphql.field.name"] == "items")
                .unwrap();
            for record in records
                .iter()
                .filter(|r| r.values.0["graphql.field.name"] == "value")
            {
                assert_eq!(record.parent, Some(items.id));
            }
            for record in records.iter() {
                assert_eq!(
                    record.values.0["graphql.outcome"],
                    if record.values.0["graphql.field.name"] == "failure" {
                        "error"
                    } else {
                        "ok"
                    }
                );
            }
            let telemetry = format!("{records:?}");
            for secret in [
                "PrivateOperation",
                "secretAlias",
                "sensitive-argument",
                "sensitive error message",
            ] {
                assert!(!telemetry.contains(secret));
            }
        }
        capture.0.lock().unwrap().clear();
        let introspection =
            futures::executor::block_on(schema.execute("{ __schema { queryType { name } } }"));
        assert!(introspection.errors.is_empty());
        let records = capture.0.lock().unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].values.0["operation.name"], "graphql.execute");
    }
}
