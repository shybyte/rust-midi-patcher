use std::time::{Duration, Instant};
use std::thread;


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbsoluteSleep {
    start_time: Instant,
    duration_sum: Duration
}

impl AbsoluteSleep {
    pub fn new() -> AbsoluteSleep {
        AbsoluteSleep { start_time: Instant::now(), duration_sum: Duration::new(0, 0) }
    }
    pub fn sleep(&mut self, duration: Duration) {
        self.duration_sum += duration;
        let now = Instant::now();
        let sleep_time = self.duration_sum - (now - self.start_time);
        thread::sleep(sleep_time);
    }
}




#[cfg(test)]
mod tests {
    use std::i64;
    use std::time::{Duration, Instant};
    use absolute_sleep::{AbsoluteSleep};
    use std::thread;

    #[test]
    fn sleep() {
        let mut abs_sleep = AbsoluteSleep::new();
        let time1 = Instant::now();
        abs_sleep.sleep(Duration::from_millis(100));
        let sleep_time: i64 = (Instant::now() - time1).subsec_nanos() as i64 - 100 * 1000_000;
        assert!(sleep_time.abs() < 1000_000, format!("sleep right time {}", sleep_time));
    }

    #[test]
    fn sleep_more() {
        let mut abs_sleep = AbsoluteSleep::new();
        let time1 = Instant::now();
        abs_sleep.sleep(Duration::from_millis(100));
        abs_sleep.sleep(Duration::from_millis(100));
        let sleep_time: i64 = (Instant::now() - time1).subsec_nanos() as i64 - 200 * 1000_000;
        assert!(sleep_time.abs() < 1000_000, format!("sleep right time {}", sleep_time));
    }

    #[test]
    fn sleep_less_if_time_has_elapsed() {
        let mut abs_sleep = AbsoluteSleep::new();
        thread::sleep(Duration::from_millis(75));
        let time1 = Instant::now();
        abs_sleep.sleep(Duration::from_millis(100));
        let sleep_time: i64 = (Instant::now() - time1).subsec_nanos() as i64 - 25 * 1000_000;
        assert!(sleep_time.abs() < 1000_000, format!("sleep right time {}", sleep_time));
    }
}
