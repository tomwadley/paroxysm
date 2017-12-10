mod fetch;
mod parse;
mod calculate;
mod vibration;
mod frequency;

extern crate chrono;

use self::chrono::prelude::*;
use std::thread;
use std::time::Duration;
use common::control::{Controller,GatttoolController};

const PULSE_DURATION_MS: u64 = 200;

#[derive(PartialEq, Debug)]
pub struct Neo {
    designation: String,
    ca_time: DateTime<Utc>,
    velocity: f64,
    magnitude: f64,
}

fn run(bd_addr: &str) {
    let mut controller = GatttoolController::new(bd_addr, 0);

    for_each_ca(|neo| {
        let vibration = vibration::from_momentum(neo.momentum_kg_km_s());
        let mut seconds_until_ca = neo.seconds_until_ca(Utc::now());

        while seconds_until_ca > 0 {
            let frequency = frequency::from_seconds_ms(seconds_until_ca);

            controller.set_vibration(vibration);
            thread::sleep(Duration::from_millis(PULSE_DURATION_MS));
            controller.set_vibration(0);
            thread::sleep(Duration::from_millis(frequency));

            seconds_until_ca = neo.seconds_until_ca(Utc::now());
        }
    })
}

fn for_each_ca<F>(mut f: F) where F: FnMut(&Neo) {
    loop {
        let now = Utc::now();

        if let Some(neos) = fetch::fetch().map(parse::parse) {
            if let Some(neo) = Neo::next_ca(&neos, now) {
                f(neo);
            } else {
                println!("No results! Sleeping and trying again.");
                thread::sleep(Duration::from_secs(60));
            }
        } else {
            println!("Failed to fetch data. Sleeping and trying again.");
            thread::sleep(Duration::from_secs(10));
        }
    }
}