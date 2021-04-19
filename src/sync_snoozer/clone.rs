use super::SyncSnoozer;
use std::sync::atomic::{AtomicUsize, Ordering};

impl Clone for SyncSnoozer {
    fn clone(&self) -> Self {
        Self {
            steps: self.steps.clone(),
            cursor: AtomicUsize::new(self.cursor.load(Ordering::SeqCst)),
            sleeper: self.sleeper.clone(),
        }
    }
}
