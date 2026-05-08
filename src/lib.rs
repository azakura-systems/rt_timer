use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Timer {
    hz: u64,
    dt: f64,
    time: f64,
    period: Duration,
    next_tick: Instant,
    gates: HashMap<u64, u64>,
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
            gates: HashMap::new(),
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

    #[inline]
    pub fn should_log(&mut self, log_hz: u64) -> bool {
        let timer_hz = self.hz;
        let acc = self.gates.entry(log_hz).or_insert(0);
        *acc += log_hz;
        if *acc >= timer_hz {
            *acc -= timer_hz;
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
