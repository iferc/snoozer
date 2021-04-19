#[cfg(test)]
mod tests;

mod builder;
mod clone;
mod debug;
mod snoozer;

use std::future::Future;
use std::sync::atomic::AtomicUsize;
use std::time::Duration;

pub struct AsyncSnoozer<F>
where
    F: Future,
{
    steps: Vec<Duration>,
    cursor: AtomicUsize,
    sleeper: fn(Duration) -> F,
}

#[derive(Clone, Debug)]
pub struct AsyncSnoozerBuilder<F>
where
    F: Future,
{
    steps: Option<Vec<Duration>>,
    sleeper: Option<fn(Duration) -> F>,
}
