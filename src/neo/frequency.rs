use std::time::Duration;

const MIN_SECONDS: f64 = 60.0;
const MAX_SECONDS: f64 = 60.0 * 60.0 * 10.0; // 10 hours
const MIN_FREQUENCY_MS: f64 = 200.0;
const MAX_FREQUENCY_MS: f64 = 5000.0;

pub fn from_seconds_ms(s: i64) -> u64 {
    let sf = s as f64;

    if sf < MIN_SECONDS {
        return MIN_FREQUENCY_MS as u64;
    }
    if sf > MAX_SECONDS {
        return MAX_FREQUENCY_MS as u64;
    }

    let result = MIN_FREQUENCY_MS
        + ((sf / MIN_SECONDS).log10() / (MAX_SECONDS / MIN_SECONDS).log10())
        * (MAX_FREQUENCY_MS - MIN_FREQUENCY_MS);

    result.round() as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn boundaries() {
        assert_eq!(super::from_seconds_ms(60), 200);
        assert_eq!(super::from_seconds_ms(36000), 5000);
    }

    #[test]
    fn out_of_bounds() {
        assert_eq!(super::from_seconds_ms(50), 200);
        assert_eq!(super::from_seconds_ms(36010), 5000);
    }

    #[test]
    fn various_values() {
        assert_eq!(super::from_seconds_ms(300), 1408); // 5 minutes
        assert_eq!(super::from_seconds_ms(3600), 3272); // 1 hour
        assert_eq!(super::from_seconds_ms(18000), 4480); // 5 hours
    }
}