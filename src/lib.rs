mod fetch;
mod parse;
mod convert;
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

    let fetcher = Fetcher::new(&api_key);
    let mut controller = Controller::new(&bd_addr, 0);

    let (s, r) = chan::async();
    let tick = chan::tick(Duration::from_secs(60));

    update(&fetcher, &mut controller);

    loop {
        chan_select! {
            tick.recv() => s.send(()),
            r.recv() => update(&fetcher, &mut controller),
        }
    }
}

fn update(fetcher: &Fetcher, controller: &mut Controller) {
    println!("start time: {:?}", std::time::Instant::now());

    if let Some(json) = fetcher.fetch_mom_json() {
        let latest_mom = parse::latest_mom(json);
        println!("Latest MOM: {}", latest_mom);

        let target_vibration = convert::mom_to_vibration(latest_mom);
        controller.set_vibration(target_vibration);
        println!("New target vibration: {}", target_vibration);
    } else {
        println!("Fetch failed!");
    }
}