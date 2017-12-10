
const MAX_VIB: u8 = 20;
const MIN_VIB: u8 = 0;

const MAX_MOM: f64 = 1.0;
const MIN_MOM: f64 = -1.0;

pub fn mom_to_vibration(v: f64) -> u8 {
    let v = -v;

    if v > MAX_MOM {
        return MAX_VIB
    }
    if v < MIN_MOM {
        return MIN_VIB
    }

    ((v - MIN_MOM) / (MAX_MOM - MIN_MOM) * ((MAX_VIB - MIN_VIB) as f64) + (MIN_VIB as f64)).round() as u8
}

#[cfg(test)]
mod tests {
    #[test]
    fn boundaries() {
        assert_eq!(super::mom_to_vibration(1.0), 0);
        assert_eq!(super::mom_to_vibration(-1.0), 20);
    }

    #[test]
    fn middle_value() {
        assert_eq!(super::mom_to_vibration(0.0), 10);
    }

    #[test]
    fn quarter_values() {
        assert_eq!(super::mom_to_vibration(0.5), 5);
        assert_eq!(super::mom_to_vibration(-0.5), 15);
    }

    #[test]
    fn out_of_bounds() {
        assert_eq!(super::mom_to_vibration(1.2), 0);
        assert_eq!(super::mom_to_vibration(-1.2), 20);
    }
}