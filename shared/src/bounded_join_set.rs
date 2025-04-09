//! A bounded version of tokio's `JoinSet` that limits the number of concurrent tasks.

use std::{future::Future, sync::Arc};
use tokio::{
    sync::Semaphore,
    task::{AbortHandle, JoinError, JoinSet},
};

/// A bounded version of tokio's `JoinSet` that limits the number of concurrent tasks.
///
/// This type wraps a `tokio::task::JoinSet` and adds a semaphore to limit the number of
/// concurrent tasks that can be spawned. When the maximum number of tasks is reached,
/// subsequent spawn attempts will block until a task completes.
#[derive(Debug)]
pub struct BoundedJoinSet<T> {
    inner: JoinSet<T>,
    semaphore: Arc<Semaphore>,
}

impl<T: 'static> BoundedJoinSet<T> {
    /// Creates a new [`BoundedJoinSet`] with the specified maximum number of concurrent tasks.
    pub fn new(max_tasks: usize) -> Self {
        Self { inner: JoinSet::new(), semaphore: Arc::new(Semaphore::new(max_tasks)) }
    }

    /// Spawns a new task into the set.
    ///
    /// If the maximum number of tasks is already running, this method will block
    /// until a task completes and a slot becomes available.
    ///
    /// # Arguments
    ///
    /// * `task` - The future to spawn
    ///
    /// # Returns
    ///
    /// Returns an [`AbortHandle`] that can be used to remotely cancel the task.
    pub fn spawn<F>(&mut self, task: F) -> AbortHandle
    where
        F: Future<Output = T> + Send + 'static,
        T: Send,
    {
        let semaphore = self.semaphore.clone();
        self.inner.spawn(async move {
            #[allow(clippy::unwrap_used)] // this should never happen
            let _permit = semaphore.acquire().await.unwrap();
            task.await
        })
    }

    /// Waits for one of the tasks in the set to complete.
    ///
    /// Returns `None` if the set is empty.
    pub async fn join_next(&mut self) -> Option<Result<T, JoinError>> {
        self.inner.join_next().await
    }

    /// Aborts all tasks in the set.
    pub fn abort_all(&mut self) {
        self.inner.abort_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::{sleep, Instant};

    #[tokio::test]
    async fn test_bounded_join_set() {
        let mut set = BoundedJoinSet::new(2);

        // Spawn first two tasks - should work
        set.spawn(async {
            sleep(Duration::from_millis(100)).await;
            1
        });
        set.spawn(async {
            sleep(Duration::from_millis(100)).await;
            2
        });

        let start = Instant::now();

        // Third spawn should block until a task completes
        set.spawn(async {
            sleep(Duration::from_millis(100)).await;
            3
        });

        // Wait for first task to complete
        assert!(set.join_next().await.is_some());

        let first_task_end_duration = start.elapsed();
        assert!(
            first_task_end_duration >= Duration::from_millis(100) &&
                first_task_end_duration < Duration::from_millis(200)
        );

        // Now third spawn should complete
        assert!(set.join_next().await.is_some());
        assert!(set.join_next().await.is_some());
        let duration = start.elapsed();
        assert!(duration >= Duration::from_millis(200));
    }
}
