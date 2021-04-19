use super::AsyncSnoozer;
use std::future::Future;
use std::sync::atomic::Ordering;

impl<F> std::fmt::Debug for AsyncSnoozer<F>
where
    F: Future,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        if formatter.alternate() {
            let formatted_steps = format!("{:#?}", self.steps)
                .lines()
                .collect::<Vec<_>>()
                .join("\n    ");

            write!(
                formatter,
                "AsyncSnoozer {{\n    step: {step},\n    steps: {steps},\n}}",
                step = self.cursor.load(Ordering::SeqCst) + 1,
                steps = formatted_steps
            )
        } else {
            write!(
                formatter,
                "AsyncSnoozer {{ step: {step}, steps: {steps:?} }}",
                step = self.cursor.load(Ordering::SeqCst) + 1,
                steps = self.steps
            )
        }
    }
}
