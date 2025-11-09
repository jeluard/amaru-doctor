use tokio::sync::mpsc;

/// Sends a provider fn, usually one getting items from an iter,
/// into a background thread.
pub struct AsyncProvider<T: Send + 'static> {
    pub rx: mpsc::Receiver<T>,
}

impl<T: Send + 'static> AsyncProvider<T> {
    pub fn new<F>(r#fn: F) -> Self
    where
        T: Send + Sync + 'static,
        F: FnOnce(mpsc::Sender<T>) + Send + 'static,
    {
        let (tx, rx) = mpsc::channel(100);
        tokio::task::spawn_blocking(move || {
            r#fn(tx);
        });

        Self { rx }
    }
}
