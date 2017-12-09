mod fetch;
mod parse;
mod calculate;

extern crate chrono;
use self::chrono::prelude::*;

#[derive(PartialEq, Debug)]
pub struct Neo {
    designation: String,
    ca_time: DateTime<Utc>,
    velocity: f64,
    magnitude: f64,
}
