extern crate serde_json;
extern crate time;

use self::serde_json::Value;

pub fn latest_mom(json: Value) -> f64 {
    let mut data = parse_data(&json);
    let latest = latest_entry(&mut data);
    latest.1
}

fn parse_data(v: &Value) -> Vec<(time::Tm, f64)> {
    v["Technical Analysis: MOM"].as_object().unwrap().iter()
        .map(|(k, v)| (parse_timestamp(k), parse_mom_field(v)))
        .collect()
}

fn parse_timestamp(s: &str) -> time::Tm {
    time::strptime(s, "%Y-%m-%d %H:%M").unwrap()
}

fn parse_mom_field(v: &Value) -> f64 {
    v["MOM"].as_str().unwrap().parse::<f64>().unwrap()
}

fn latest_entry(data: &mut Vec<(time::Tm, f64)>) -> &(time::Tm, f64) {
    data.sort_unstable_by_key(|&(timestamp, _)| timestamp);
    data.last().unwrap()
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    const MOM_JSON: &'static str = include_str!("resources/alphavantage-mom.json");

    #[test]
    fn transforms() {
        let json = serde_json::from_str(MOM_JSON).unwrap();
        assert_eq!(super::latest_mom(json), -0.0100f64);
    }
}