
pub fn from_momentum(m: f64) -> u8 {
    let b = m.log10().trunc() as i32;

    if b < 6 {
        return 1;
    }
    if b == 6 || b == 7 || b == 12 {
        let i = m / 10.0f64.powi(b + 1);
        if i < (1.0 / 2.0) {
            if b == 6 { return 1 };
            if b == 7 { return 3 };
            if b == 12 { return 17 };
        } else {
            if b == 6 { return 2 };
            if b == 7 { return 4 };
            if b == 12 { return 18 };
        }
    }
    if b >= 8 && b <= 11 {
        let i = m / 10.0f64.powi(b + 1);
        if i < (1.0 / 3.0) {
            if b == 8 { return 5 };
            if b == 9 { return 8 };
            if b == 10 { return 11 };
            if b == 11 { return 14 };
        } else if i > (1.0 / 3.0 * 2.0) {
            if b == 8 { return 7 };
            if b == 9 { return 10 };
            if b == 10 { return 13 };
            if b == 11 { return 16 };
        } else {
            if b == 8 { return 6 };
            if b == 9 { return 9 };
            if b == 10 { return 12 };
            if b == 11 { return 15 };
        }
    }
    if b == 13 || b == 14 {
        return 19;
    }
    return 20;
}

#[cfg(test)]
mod tests {
    #[test]
    fn converts_momentum_to_vibration() {
        // below 6
        assert_eq!(super::from_momentum(300_000_f64), 1);

        // low 6
        assert_eq!(super::from_momentum(3_000_000_f64), 1);
        // high 6
        assert_eq!(super::from_momentum(7_000_000_f64), 2);
        // low 7
        assert_eq!(super::from_momentum(30_000_000_f64), 3);
        // high 7
        assert_eq!(super::from_momentum(70_000_000_f64), 4);
        // low 8
        assert_eq!(super::from_momentum(300_000_000_f64), 5);
        // med 8
        assert_eq!(super::from_momentum(500_000_000_f64), 6);
        // high 8
        assert_eq!(super::from_momentum(700_000_000_f64), 7);
        // low 9
        assert_eq!(super::from_momentum(3_000_000_000_f64), 8);
        // med 9
        assert_eq!(super::from_momentum(5_000_000_000_f64), 9);
        // high 9
        assert_eq!(super::from_momentum(7_000_000_000_f64), 10);
        // low 10
        assert_eq!(super::from_momentum(30_000_000_000_f64), 11);
        // med 10
        assert_eq!(super::from_momentum(50_000_000_000_f64), 12);
        // high 10
        assert_eq!(super::from_momentum(70_000_000_000_f64), 13);
        // low 11
        assert_eq!(super::from_momentum(300_000_000_000_f64), 14);
        // med 11
        assert_eq!(super::from_momentum(500_000_000_000_f64), 15);
        // high 11
        assert_eq!(super::from_momentum(700_000_000_000_f64), 16);
        // low 12
        assert_eq!(super::from_momentum(3_000_000_000_000_f64), 17);
        // high 12
        assert_eq!(super::from_momentum(7_000_000_000_000_f64), 18);
        // 13
        assert_eq!(super::from_momentum(50_000_000_000_000_f64), 19);
        // 14
        assert_eq!(super::from_momentum(500_000_000_000_000_f64), 19);
        // 15
        assert_eq!(super::from_momentum(5_000_000_000_000_000_f64), 20);

        // above 15
        assert_eq!(super::from_momentum(30_000_000_000_000_000_f64), 20);
    }
}