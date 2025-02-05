use rust_decimal::Decimal;

pub fn run() {
    let _a = Decimal::new(10, 0);
    let _b = Decimal::new(11, 0);

    let x = 1267906.43;
    let y = 1267906.4299999997;
    let z: f64 = x - y;

    println!("{} {}", z, z.abs() < 0.001);
}
