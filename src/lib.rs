use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Timer {
    hz: u64,
    dt: f64,
    time: f64,
    period: Duration,
    next_tick: Instant,
    log_accum: u64,
}

impl Timer {
    pub fn new(hz: u64) -> Self {
        let dt = 1.0 / hz as f64;
        let period = Duration::from_secs_f64(dt);
        Self {
            hz,
            dt,
            time: 0.0,
            period,
            next_tick: Instant::now() + period,
            log_accum: 0,
        }
    }

    pub fn dt(&self) -> f64 {
        self.dt
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn stamp(&mut self) -> f64 {
        self.time += self.dt;
        self.time
    }

    pub fn should_log(&mut self, hz: u64) -> bool {
        self.log_accum += hz;
        if self.log_accum >= self.hz {
            self.log_accum -= self.hz;
            true
        } else {
            false
        }
    }

    pub fn wait(&mut self) {
        let now = Instant::now();
        if now < self.next_tick {
            spin_sleep::sleep(self.next_tick - now);
            self.next_tick += self.period;
        } else {
            self.next_tick = now + self.period;
        }
    }
}
