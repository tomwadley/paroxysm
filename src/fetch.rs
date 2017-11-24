extern crate reqwest;
extern crate serde_json;

use self::serde_json::Value;
use self::reqwest::Url;

const URL: &'static str = "https://www.alphavantage.co/query";

pub struct Fetcher {
    url: Url,
}

impl Fetcher {
    pub fn new(apikey: &str) -> Fetcher {
        Fetcher { url: Url::parse_with_params(URL, vec![
            ("function", "MOM"),
            ("symbol", "SPX"),
            ("interval", "1min"),
            ("time_period", "1"),
            ("series_type", "close"),
            ("apikey", apikey)
        ]).unwrap() }
    }

    pub fn fetch_mom_json(&self) -> Option<Value> {
        println!("Fetching: {}", self.url);

        if let Ok(r) = reqwest::get(self.url.clone()).as_mut() {
            if r.status().is_success() {
                return r.json().ok();
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    extern crate config;

    #[test]
    fn fetches() {
        let api_key = load_config().get_str("alphavantage_api_key").unwrap();

        match super::Fetcher::new(&api_key).fetch_mom_json() {
            None => panic!("Something wrong - no json!"),
            Some(v) => {
                println!("JSON RESPONSE: {}", v);
                v["Technical Analysis: MOM"].as_object().expect("Field missing in response")
            },
        };
    }

    fn load_config() -> config::Config {
        let mut conf = config::Config::new();
        conf.merge(config::File::with_name("Config")).unwrap();
        conf
    }
}