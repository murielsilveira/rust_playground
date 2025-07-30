#![allow(dead_code)]

use std::path::Path;

mod sandbox;

#[tokio::main]
async fn main() {
    // sandbox::traits::run();
    // sandbox::decimals::run();
    // sandbox::defaults::run();
    // sandbox::excel::run();
    // sandbox::dates::run();
    // sandbox::csv::run();

    // use TZ=Europe/London
    // let now = Local::now();
    // println!("now: {}", now);

    let name = "directory/asdf.qwer.20250729220127.{{filename}}.txn";
    let b = Path::new(name);
    let c = b.file_name();
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", sanitize_filename_reader_friendly::sanitize(name));

    // let u = Url::from_str("ssh://localhost:2020").unwrap();
    // println!("{:?}", u.port());

    // let to = Address::new("ms", "example.com").unwrap();
    // println!("{}", to);
}
