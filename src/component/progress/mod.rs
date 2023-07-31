use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct ProgressBar {
    progress: f64,
    max: u64,
}

impl ProgressBar {
    pub fn new(max: u64) -> Self {
        Self { progress: 0., max }
    }

    pub fn new_in_progress(progress: f64, max: u64) -> Self {
        Self { progress, max }
    }

    pub fn new_finished(max: u64) -> Self {
        Self {
            progress: max as f64,
            max,
        }
    }

    pub fn percentage(&self) -> f64 {
        (self.progress / self.max as f64) * 100_f64
    }

    pub fn is_finished(&self) -> bool {
        self.progress >= self.max as f64
    }

    pub fn set_progress(&mut self, progress: f64) {
        if progress >= self.max as f64 {
            self.progress = self.max as f64;
            return;
        }
        if progress <= 0. {
            self.progress = 0.;
            return;
        }

        self.progress = progress;
    }

    pub fn get_progress(&self) -> f64 {
        self.progress
    }

    pub fn get_max(&self) -> u64 {
        self.max
    }

    pub fn restart(&mut self) {
        self.progress = 0.;
    }

    pub fn advance(&mut self) {
        self.increment(1.)
    }

    pub fn increment(&mut self, by: f64) {
        if self.progress + by >= self.max as f64 {
            self.progress = self.max as f64;
            return;
        }

        self.progress += by;
    }

    pub fn decrement(&mut self, by: f64) {
        if self.progress - by <= 0. {
            self.progress = 0.;
            return;
        }

        self.progress -= by;
    }

    pub fn finish(&mut self) {
        self.progress = self.max as f64;
    }

    pub fn as_range(&self) -> std::ops::Range<f64> {
        0.0..self.max as f64
    }
}

impl Display for ProgressBar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let max_width: usize = (self.max as f64).log10() as usize + 3;

        write!(
            f,
            "{: >max_width$}/{} ({: >3}%)",
            (self.progress * 100.0).round() / 100.0,
            self.max,
            self.percentage().round(),
            max_width = max_width
        )
    }
}
