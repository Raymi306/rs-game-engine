use std::time::Duration;

pub struct Timer {
    pub acc: Duration,
    pub length: Duration,
    pub done: bool,
}

impl Timer {
    pub fn new(length: Duration) -> Self {
        Self {
            acc: Duration::from_secs(0),
            length,
            done: false,
        }
    }
    pub fn update(&mut self, duration: Duration) {
        self.acc += duration;
        if self.acc >= self.length {
            self.done = true;
        }
    }
    pub fn restart(&mut self) {
        self.acc = Duration::from_secs(0);
        self.done = false;
    }
    pub fn set_length(&mut self, length: Duration) {
        self.length = length;
    }
    pub fn force(&mut self) {
        self.done = true;
    }
}
