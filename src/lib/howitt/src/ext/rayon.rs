use tokio::sync::oneshot;

pub async fn rayon_spawn_blocking<F, T>(f: F) -> T
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = oneshot::channel();

    rayon::spawn(move || {
        let result = f();

        tx.send(result).unwrap_or_else(|_| {
            panic!("Failed to send result from rayon task");
        });
    });

    rx.await.expect("Rayon task panicked")
}
