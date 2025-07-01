#![allow(dead_code)]

use std::path::{Path, PathBuf};

mod sandbox;

fn main() {
    // sandbox::traits::run();
    // sandbox::decimals::run();
    // sandbox::defaults::run();
    // sandbox::excel::run();
    sandbox::dates::run();
    // sandbox::csv::run();

    let b = Path::new("../test.csv");

    println!("{:?}", b.file_name().map(|n| n.to_ascii_lowercase()));
}
