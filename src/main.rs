#![allow(dead_code, unused)]

use std::{path::Path, str::FromStr};

use chrono::Local;
use lettre::Address;
use url::Url;

mod sandbox;

fn main() {
    // sandbox::traits::run();
    // sandbox::decimals::run();
    // sandbox::defaults::run();
    // sandbox::excel::run();
    // sandbox::dates::run();
    sandbox::csv::run();

    // use TZ=Europe/London
    // let now = Local::now();
    // println!("now: {}", now);

    // let b = Path::new("../test.csv");
    // println!("{:?}", b.file_name().map(|n| n.to_ascii_lowercase()));

    // let u = Url::from_str("ssh://localhost:2020").unwrap();
    // println!("{:?}", u.port());

    // let to = Address::new("ms", "example.com").unwrap();
    // println!("{}", to);
}
