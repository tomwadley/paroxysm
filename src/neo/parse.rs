extern crate serde_json;

use neo::Neo;
use self::serde_json::Value;
use neo::chrono::prelude::*;
use std::collections::HashMap;
use std::iter::FromIterator;

const NEO_CA_VERSION: &'static str = "1.1";

type FieldIndices = HashMap<String, usize>;

pub fn parse(json: Value) -> Vec<Neo> {
    check_version(&json);
    let fi = get_field_indices(&json);
    json["data"].as_array().unwrap().iter()
        .map(|v| parse_entry(v, &fi)).collect()
}

fn get_field_indices(json: &Value) -> FieldIndices {
    let fields: Vec<&str> = json["fields"].as_array().unwrap().iter()
        .map(|v| v.as_str().unwrap()).collect();
    let val_index_pairs: Vec<(String, usize)> = fields.iter().enumerate()
        .map(|(i, &v)| (String::from(v), i)).collect();
    HashMap::from_iter(val_index_pairs)
}

fn parse_entry(v: &Value, fi: &FieldIndices) -> Neo {
    Neo {
        designation: parse_string(&v[fi["des"]]),
        ca_time: parse_datetime(&v[fi["cd"]]),
        velocity: parse_float(&v[fi["v_rel"]]),
        magnitude: parse_float(&v[fi["h"]]),
    }
}

fn parse_string(v: &Value) -> String {
    String::from(v.as_str().unwrap())
}

fn parse_datetime(v: &Value) -> DateTime<Utc> {
    // TDB is close enough to UTC for our purposes
    Utc.datetime_from_str(v.as_str().unwrap(), "%Y-%b-%d %H:%M").unwrap()
}

fn parse_float(v: &Value) -> f64 {
    v.as_str().unwrap().parse().unwrap()
}

fn check_version(json: &Value) {
    let version = json["signature"]["version"].as_str().unwrap();
    if version != NEO_CA_VERSION {
        println!("WARNING: neo ca json version unexpected: {}", version);
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use neo::chrono::prelude::*;

    const NEO_JSON: &'static str = include_str!("resources/test/neo-ca.json");

    #[test]
    fn transforms() {
        let json = serde_json::from_str(NEO_JSON).unwrap();

        assert_eq!(super::parse(json).first().unwrap(), &super::Neo {
            designation: String::from("2017 WP15"),
            ca_time: Utc.ymd(2017, 12, 2).and_hms(3, 30, 0),
            velocity: 20.4414586660904,
            magnitude: 24.754,
        });
    }
}