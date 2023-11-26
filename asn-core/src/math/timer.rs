use web_time::{Duration, Instant};

pub struct Timer {
    time: Instant,
    delta: Duration,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            delta: Duration::default(),
        }
    }
}

impl Timer {
    pub fn reset(&mut self) {
        self.time = Instant::now();
        self.delta = Duration::default();
    }
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.time;
        self.time = now
    }
    pub fn duration(&self) -> Duration {
        self.delta
    }
}
