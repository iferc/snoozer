use super::SyncSnoozer;
use std::sync::atomic::Ordering;

impl SyncSnoozer {
    pub fn sleep(&self) -> &Self {
        let time = self.steps[self.cursor.load(Ordering::SeqCst)];
        (self.sleeper)(time);
        self
    }

    pub fn first_step(&self) -> &Self {
        self.cursor.store(0, Ordering::Relaxed);
        self
    }

    pub fn last_step(&self) -> &Self {
        let max = self.steps.len() - 1;
        self.cursor.store(max, Ordering::Relaxed);
        self
    }

    pub fn increment_step(&self) -> &Self {
        if self.cursor.load(Ordering::SeqCst) < self.steps.len() - 1 {
            self.cursor.fetch_add(1, Ordering::Relaxed);
        }
        self
    }

    pub fn decrement_step(&self) -> &Self {
        if self.cursor.load(Ordering::SeqCst) > 0 {
            self.cursor.fetch_sub(1, Ordering::Relaxed);
        }
        self
    }
}
