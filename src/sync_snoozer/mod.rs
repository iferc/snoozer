#[cfg(test)]
mod tests;

mod builder;
mod clone;
mod debug;
mod snoozer;

use std::sync::atomic::AtomicUsize;
use std::time::Duration;

pub struct SyncSnoozer {
    steps: Vec<Duration>,
    cursor: AtomicUsize,
    sleeper: fn(Duration) -> (),
}

#[derive(Clone, Debug)]
pub struct SyncSnoozerBuilder {
    steps: Option<Vec<Duration>>,
    sleeper: Option<fn(Duration) -> ()>,
}
