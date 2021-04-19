use super::{AsyncSnoozer, AsyncSnoozerBuilder};
use std::future::Future;
use std::sync::atomic::AtomicUsize;
use std::time::Duration;

impl<F> AsyncSnoozerBuilder<F>
where
    F: Future,
{
    pub fn new() -> Self {
        Self {
            steps: None,
            sleeper: None,
        }
    }

    pub fn sleeper(mut self, sleeper: fn(Duration) -> F) -> Self {
        self.sleeper = Some(sleeper);
        self
    }

    pub fn steps_millis(mut self, steps: Vec<u64>) -> Self {
        let duration_steps = steps
            .iter()
            .map(|millis_step| Duration::from_millis(*millis_step))
            .collect::<Vec<_>>();
        self.steps = Some(duration_steps);
        self
    }

    pub fn steps_duration(mut self, steps: Vec<Duration>) -> Self {
        self.steps = Some(steps);
        self
    }

    pub fn build(self) -> Result<AsyncSnoozer<F>, ()> {
        match (self.steps, self.sleeper) {
            (Some(steps), Some(sleeper)) if steps.len() != 0 => Ok(AsyncSnoozer {
                steps,
                cursor: AtomicUsize::new(0),
                sleeper,
            }),
            _ => Err(()),
        }
    }
}
