extern crate reqwest;
extern crate serde_json;
extern crate time;

use self::serde_json::Value;
use self::reqwest::Url;
use neo::chrono::prelude::*;
use self::time::Duration;

const URL: &'static str = "https://ssd-api.jpl.nasa.gov/cad.api";
const DIST_MAX: &'static str = "0.2";
const DAYS_OF_DATA: i64 = 7;

pub fn fetch() -> Option<Value> {
    let url = build_url(Utc::now());
    println!("Fetching: {}", url);

    if let Ok(r) = reqwest::get(url.clone()).as_mut() {
        if r.status().is_success() {
            return r.json().ok();
        }
    }
    None
}

fn build_url(now: DateTime<Utc>) -> Url {
    Url::parse_with_params(URL, vec![
        ("dist-max", DIST_MAX),
        ("date-max", &(now + Duration::days(DAYS_OF_DATA)).format("%Y-%m-%d").to_string()),
    ]).unwrap()
}

#[cfg(test)]
mod tests {
    use neo::chrono::prelude::*;

    #[test]
    fn fetches() {
        match super::fetch() {
            None => panic!("Something wrong - no json!"),
            Some(v) => {
                println!("JSON RESPONSE: {}", v);
                assert_eq!(v["signature"]["version"].as_str().unwrap(), "1.1");
                v["fields"].as_array().expect("Field missing in response");
                v["data"].as_array().expect("Field missing in response");
            },
        };
    }

    #[test]
    fn builds_url() {
        let now = Utc.ymd(2017, 12, 10).and_hms(12, 0, 0);
        assert_eq!(format!("{}", super::build_url(now)),
                   "https://ssd-api.jpl.nasa.gov/cad.api?dist-max=0.2&date-max=2017-12-17");
    }
}