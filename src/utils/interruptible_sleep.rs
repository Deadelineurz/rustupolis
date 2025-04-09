use std::sync::{Condvar, Mutex};
use std::time::Duration;

#[macro_export]
macro_rules! return_on_cancel {
    ($var: expr, $dur: expr) => {
        let res = $var.wait_for($dur);
        if !res {
            return
        }
    };
}

#[derive(Debug)]
pub struct InterruptibleSleep {
    mtx: Mutex<bool>,
    cond: Condvar
}

impl InterruptibleSleep {
    pub fn new() -> Self {
        Self {
            mtx: Mutex::new(false),
            cond: Condvar::new()
        }
    }

    /// Returns whether the wait ended after duration (true) or not (false)
    pub fn wait_for(&self, duration: Duration) -> bool {
        let lock = self.mtx.lock().unwrap();
        if lock.clone() {
            false
        } else if let Err(x) = self.cond.wait_timeout(lock, duration) {
            !x.get_ref().1.timed_out()
        } else {
            true
        }
    }

    pub fn stop(&self) {
        *self.mtx.lock().unwrap() = true;
        self.cond.notify_all()
    }
}