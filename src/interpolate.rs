use std::time::Duration;
use std::ops::Div;
use std::thread;

pub fn interpolate<F>(from: u8, to: u8, duration: Duration, mut f: F) where F: FnMut(u8) {
    let (interval, step) = interval_and_step(from, to, duration);

    let mut i = from;

    while i != to {
        i = (i as i32 + step as i32) as u8;

        f(i);

        if i != to {
            thread::sleep(interval);
        }
    }
}

fn interval_and_step(from: u8, to: u8, duration: Duration) -> (Duration, i8) {
    let diff = to as i16 - from as i16;

    if diff == 0 {
        (Duration::default(), 0)
    } else if diff > 0 {
        (duration.div(diff as u32), 1)
    } else {
        (duration.div((diff * -1) as u32), -1)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration,Instant};
    use std::ops::{Add,Mul};

    #[test]
    fn steps_up() {
        assert_eq!(super::interval_and_step(4, 14, Duration::from_secs(60)), (Duration::from_secs(6), 1));
        test_interpolate(4, 14, vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    }

    #[test]
    fn steps_down() {
        assert_eq!(super::interval_and_step(8, 2, Duration::from_secs(60)), (Duration::from_secs(10), -1));
        test_interpolate(8, 2, vec![7, 6, 5, 4, 3, 2]);
    }

    #[test]
    fn no_step() {
        assert_eq!(super::interval_and_step(9, 9, Duration::from_secs(60)), (Duration::from_secs(0), 0));
        test_interpolate(9, 9, vec![]);
    }

    fn test_interpolate(from: u8, to: u8, expected_steps: Vec<u8>) {
        let mut steps = vec![];
        let (interval, _) = super::interval_and_step(from, to, Duration::from_secs(1));
        let start_time = Instant::now();

        super::interpolate(from, to, Duration::from_secs(1), |i| steps.push(i));

        let end_time = Instant::now();
        assert_eq!(steps, expected_steps);
        if expected_steps.len() > 0 {
            assert!(end_time.ge(&start_time.add(interval.mul((expected_steps.len() - 1) as u32))));
            assert!(end_time.lt(&start_time.add(interval.mul(expected_steps.len() as u32))));
        }
    }
}