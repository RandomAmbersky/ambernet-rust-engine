use std::time::Instant;

pub struct Fps {
    last_time: Instant,
    throttle: f32,
}

impl Fps {
    pub fn new(throttle: f32) -> Self {
        Self {
            last_time: Instant::now(),
            throttle,
        }
    }
    pub fn time(&self) -> Instant {
        self.last_time
    }
    pub fn tick(&mut self) -> bool {
        let now = Instant::now();
        let delta = now - self.last_time;

        if delta.as_secs_f32() > self.throttle {
            self.last_time = now;
            return true;
        }
        false
    }
}
