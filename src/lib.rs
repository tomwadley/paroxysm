mod fetch;
mod parse;
mod convert;
mod interpolate;
mod control;

#[macro_use]
extern crate chan;
extern crate config;

use std::time::Duration;
use fetch::Fetcher;
use control::Controller;

pub fn start() {
    let mut conf = config::Config::new();
    conf.merge(config::File::with_name("Config")).expect("Failed to load config!");

    let bd_addr = conf.get_str("bd_addr").unwrap();
    let api_key = conf.get_str("alphavantage_api_key").unwrap();

    let mut stock_vibrate = StockVibrate::new(&bd_addr, &api_key);
    stock_vibrate.update_loop();
}

struct StockVibrate {
    vibration: u8,
    fetcher: Fetcher,
    controller: Controller,
}

impl StockVibrate {
    fn new(bd_addr: &str, api_key: &str) -> StockVibrate {
        StockVibrate {
            vibration: 0,
            fetcher: Fetcher::new(api_key),
            controller: Controller::new(bd_addr, 0)
        }
    }

    fn update_loop(&mut self) {
        let (s, r) = chan::async();
        let tick = chan::tick(Duration::from_secs(60));

        self.update(false);

        loop {
            chan_select! {
                tick.recv() => s.send(()),
                r.recv() => self.update(true),
            }
        }
    }

    fn update(&mut self, use_interpolation: bool) {
        if let Some(json) = self.fetcher.fetch_mom_json() {
            let latest_mom = parse::latest_mom(json);
            println!("Latest MOM: {}", latest_mom);

            let target_vibration = convert::mom_to_vibration(latest_mom);

            if use_interpolation {
                println!("Stepping to vibration: {}", target_vibration);

                interpolate::interpolate(self.vibration, target_vibration, Duration::from_secs(60), |v| {
                    println!("Vibration: {}", v);
                    self.controller.set_vibration(v);
                });
            } else {
                println!("Setting vibration to: {}", target_vibration);
                self.controller.set_vibration(target_vibration);
            }

            self.vibration = target_vibration;

        } else {
            println!("Fetch failed!");
        }
    }
}