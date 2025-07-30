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

    let name = "{{filename}}.txn.csv";
    let full = format!("directory/{}", name);
    let path = Path::new(&full);
    let sanitized = sanitize_filename_reader_friendly::sanitize(name);
    println!("Path:           {:?}", path);
    println!("Path file name: {:?}", path.file_name());
    println!("Sanitized:      {:?}", sanitized);

    // let u = Url::from_str("ssh://localhost:2020").unwrap();
    // println!("{:?}", u.port());

    // let to = Address::new("ms", "example.com").unwrap();
    // println!("{}", to);
}
