use futures::{prelude::*, stream::FuturesOrdered, Future};

pub trait FuturesIteratorExt<T> {
    async fn collect_futures_ordered(self) -> Vec<T>;
}

impl<I, F, T> FuturesIteratorExt<T> for I
where
    I: Iterator<Item = F>,
    F: Future<Output = T>,
{
    async fn collect_futures_ordered(self) -> Vec<T> {
        self.collect::<FuturesOrdered<_>>().collect().await
    }
}
