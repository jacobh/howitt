use tokio::sync::oneshot;

pub async fn rayon_spawn_blocking<F, T>(f: F) -> T
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = oneshot::channel();

    rayon::spawn(move || {
        let result = f();
        let _ = tx.send(result);
    });

    rx.await.expect("Rayon task panicked")
}
