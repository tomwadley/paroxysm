extern crate vibrate_experiment;
extern crate config;

use vibrate_experiment::market_momentum::MarketMomentum;

fn main() {
    let mut conf = config::Config::new();
    conf.merge(config::File::with_name("Config")).expect("Failed to load config!");

    let bd_addr = conf.get_str("bd_addr").unwrap();
    let api_key = conf.get_str("alphavantage_api_key").unwrap();

    let mut market_momentum = MarketMomentum::new(&bd_addr, &api_key);
    market_momentum.start();
}
