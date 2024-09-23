use std::collections::HashMap;

use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::{BLUE, ORANGE};

pub fn progress_style(prefix: &str) -> ProgressStyle {
    let template = "{spinner:.cyan} {msg}";
    ProgressStyle::with_template(&format!("{prefix}{template}")).unwrap()
}

#[derive(PartialEq, Eq, Hash)]
pub enum Step {
    Resolve,
    Fetch,
    Extract,
}

pub struct Progress(MultiProgress, HashMap<Step, ProgressBar>);
impl Progress {
    pub fn new() -> Self {
        Self(MultiProgress::new(), HashMap::new())
    }

    pub fn start_single(&self, msg: String) -> ProgressBar {
        let msg = style(msg).dim().to_string();
        let pb = self.0.add(
            ProgressBar::new_spinner()
                .with_style(progress_style("    "))
                .with_message(msg),
        );

        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb
    }

    pub fn finish_single(bar: ProgressBar, msg: String) {
        // bar.finish_and_clear();
        bar.finish_with_message(style(msg).color256(BLUE).to_string());
    }

    pub fn abandon_single(bar: ProgressBar, msg: String) {
        bar.abandon_with_message(style(msg).color256(ORANGE).to_string());
    }

    pub fn start(&mut self, step: Step) {
        let msg = match step {
            Step::Resolve => style("Resolving assets...").to_string(),
            Step::Fetch => style("Fetching assets...").to_string(),
            Step::Extract => style("Unpacked assets...").to_string(),
        };

        let style = match step {
            Step::Resolve | Step::Fetch | Step::Extract => progress_style(""),
        };

        let pb = self.0.add(
            ProgressBar::new_spinner()
                .with_style(style)
                .with_message(msg),
        );
        pb.enable_steady_tick(std::time::Duration::from_millis(100));

        self.1.insert(step, pb);
    }

    pub fn finish(&self, step: Step) {
        let msg = match step {
            Step::Resolve => style("Resolved assets").color256(BLUE).to_string(),
            Step::Fetch => style("Fetched assets").color256(BLUE).to_string(),
            Step::Extract => style("Unpacked assets").color256(BLUE).to_string(),
        };

        if let Some(pb) = self.1.get(&step) {
            pb.finish_with_message(msg);
        }
    }

    pub fn abandon(&self, step: Step, error: Box<dyn std::error::Error>) {
        let msg = match step {
            Step::Resolve => style(format!("Resolving assets failed: {error}"))
                .color256(ORANGE)
                .to_string(),
            Step::Fetch => style(format!("Resolving assets failed: {error}"))
                .color256(ORANGE)
                .to_string(),
            Step::Extract => style(format!("Resolving assets failed: {error}"))
                .color256(ORANGE)
                .to_string(),
        };

        if let Some(pb) = self.1.get(&step) {
            pb.finish_with_message(msg);
        }
    }
}
