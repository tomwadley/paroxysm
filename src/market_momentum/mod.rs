mod fetch;
mod parse;
mod convert;
mod interpolate;

extern crate chan;

use std::time::Duration;
use self::fetch::Fetcher;
use common::control::{Controller,GatttoolController};

pub struct MarketMomentum {
    vibration: u8,
    fetcher: Fetcher,
    controller: GatttoolController,
}

impl MarketMomentum {
    pub fn new(bd_addr: &str, api_key: &str) -> MarketMomentum {
        MarketMomentum {
            vibration: 0,
            fetcher: Fetcher::new(api_key),
            controller: GatttoolController::new(bd_addr, 0),
        }
    }

    pub fn start(&mut self) {
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