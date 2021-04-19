use super::SyncSnoozer;
use std::sync::atomic::Ordering;

impl std::fmt::Debug for SyncSnoozer {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        if formatter.alternate() {
            let formatted_steps = format!("{:#?}", self.steps)
                .lines()
                .collect::<Vec<_>>()
                .join("\n    ");

            write!(
                formatter,
                "SyncSnoozer {{\n    step: {step},\n    steps: {steps},\n}}",
                step = self.cursor.load(Ordering::SeqCst) + 1,
                steps = formatted_steps
            )
        } else {
            write!(
                formatter,
                "SyncSnoozer {{ step: {step}, steps: {steps:?} }}",
                step = self.cursor.load(Ordering::SeqCst) + 1,
                steps = self.steps
            )
        }
    }
}
