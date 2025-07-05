use std::{sync::Arc, time::Duration};
use tokio::{sync::Semaphore, task};

pub struct RateLimiter {
    bucket: Arc<Semaphore>,
}

impl RateLimiter {
    pub fn new(max_per_sec: usize) -> Self {
        let bucket = Arc::new(Semaphore::new(max_per_sec));
        let bucket_clone = bucket.clone();

        task::spawn(async move {
            let interval = tokio::time::interval(Duration::from_secs(1));
            tokio::pin!(interval);
            loop {
                interval.as_mut().tick().await;
                let permits_to_fill = max_per_sec - bucket_clone.available_permits();
                bucket_clone.add_permits(permits_to_fill);
            }
        });

        Self { bucket }
    }

    pub fn allow(&self) -> bool {
        self.bucket.try_acquire().is_ok()
    }
}
