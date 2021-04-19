use super::AsyncSnoozer;
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};

impl<F> Clone for AsyncSnoozer<F>
where
    F: Future,
{
    fn clone(&self) -> Self {
        Self {
            steps: self.steps.clone(),
            cursor: AtomicUsize::new(self.cursor.load(Ordering::SeqCst)),
            sleeper: self.sleeper.clone(),
        }
    }
}
