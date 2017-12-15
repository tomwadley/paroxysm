extern crate paroxysm;
extern crate config;
#[macro_use]
extern crate clap;

use paroxysm::market_momentum::MarketMomentum;
use paroxysm::morse;
use paroxysm::neo;
use clap::{ App, SubCommand};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("market"))
        .subcommand(SubCommand::with_name("morse"))
        .subcommand(SubCommand::with_name("neo"))
        .get_matches();

    let mut conf = config::Config::new();
    conf.merge(config::File::with_name("Config")).expect("Failed to load config!");

    let bd_addr = conf.get_str("bd_addr").unwrap();

    match matches.subcommand_name() {
        Some("market") => {
            let api_key = conf.get_str("alphavantage_api_key").unwrap();

            let mut market_momentum = MarketMomentum::new(&bd_addr, &api_key);
            market_momentum.start();
        },
        Some("morse") => {
            morse::morse(&bd_addr);
        },
        Some("neo") => {
            neo::run(&bd_addr);
        }
        _ => println!("{}", matches.usage())
    }
}
